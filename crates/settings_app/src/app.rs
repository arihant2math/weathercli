use dark_light::Mode;
use iced::widget::{button, checkbox, column, container, radio, row, text, text_input, toggler};
use iced::{Alignment, Element, Length, Sandbox, Theme};
use log::error;
use rfd::FileDialog;

use local::settings;

use crate::datasource::DataSource;
use crate::message::Message;

pub(crate) struct App {
    theme: Theme,
    data: settings::Settings,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let mode = dark_light::detect();
        let theme = match mode {
            Mode::Default => Theme::default(),
            Mode::Light => Theme::Light,
            Mode::Dark => Theme::Dark,
        };
        let data = settings::Settings::new().expect("Loading settings failed");
        App { theme, data }
    }

    fn title(&self) -> String {
        String::from("Settings - WeatherCLI")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Save => self
                .data
                .write()
                .unwrap_or(error!("Saving settings failed")),
            Message::Cancel => {
                self.data = settings::Settings::new().expect("Loading settings failed")
            }
            Message::PickLayoutFile => {
                let file = FileDialog::new()
                    .add_filter("Layout File", &["res"])
                    .set_directory(weather_dirs::layouts_dir().unwrap())
                    .pick_file();
                if let Some(real_file) = file {
                    let parent = real_file.parent().unwrap();
                    if parent == weather_dirs::layouts_dir().unwrap() {
                        self.data.layout_file =
                            real_file.file_name().unwrap().to_str().unwrap().to_string();
                    }
                }
            }
            Message::ConstantLocation(value) => self.data.constant_location = value,
            Message::MetricDefault(value) => self.data.metric_default = value,
            Message::ShowAlerts(value) => self.data.show_alerts = value,
            Message::AutoUpdateInternetResources(value) => {
                self.data.auto_update_internet_resources = value
            }
            Message::OpenWeatherMapAPIKey(value) => self.data.open_weather_map_api_key = value,
            Message::OpenWeatherMapOneCallKey(value) => {
                self.data.open_weather_map_one_call_key = value
            }
            Message::BingMapsAPIKey(value) => self.data.bing_maps_api_key = value,
            Message::DataSource(value) => {
                self.data.default_backend = value.to_string().to_uppercase()
            }
            Message::LayoutFile(value) => self.data.layout_file = value,
        }
    }

    fn view(&self) -> Element<Message> {
        let data_source = [
            DataSource::Meteo,
            DataSource::OpenWeatherMap,
            DataSource::OpenWeatherMapOneCall,
            DataSource::Nws,
        ]
        .iter()
        .fold(
            column![text("Default Backend:")].spacing(10),
            |column, data_source| {
                column.push(radio(
                    format!("{data_source:?}"),
                    *data_source,
                    Some(match &*self.data.default_backend.clone().to_lowercase() {
                        "openweathermap" => DataSource::OpenWeatherMap,
                        "openweathermap onecall" => DataSource::OpenWeatherMapOneCall,
                        "openweathermap_onecall" => DataSource::OpenWeatherMapOneCall,
                        "nws" => DataSource::Nws,
                        _ => DataSource::Meteo,
                    }),
                    Message::DataSource,
                ))
            },
        );

        let layout_file_label = text("Layout File: ");

        let layout_file = text_input("Layout File", &self.data.layout_file.clone())
            .on_input(Message::LayoutFile)
            .padding(10);

        let pick_layout_file = button("Pick Layout File")
            .padding(10)
            .on_press(Message::PickLayoutFile);
        let api_keys = text("API Keys").size(30);
        let backend = text("Backend").size(30);
        let general = text("General").size(30);
        let openweathermap_api_key_label = text("OpenWeatherMap API key: ");

        let openweathermap_api_key = text_input(
            "OpenWeatherMap API key",
            &self.data.open_weather_map_api_key.clone(),
        )
        .on_input(Message::OpenWeatherMapAPIKey)
        .padding(10);

        let one_call = checkbox(
            String::from("OneCall Compatable"),
            self.data.open_weather_map_one_call_key,
        )
        .on_toggle(Message::OpenWeatherMapOneCallKey)
        .width(Length::Shrink)
        .spacing(10);

        let bing_maps_api_key_label = text("Bing Maps API key: ");
        let bing_maps_api_key =
            text_input("Bing Maps API key", &self.data.bing_maps_api_key.clone())
                .on_input(Message::BingMapsAPIKey)
                .padding(10);

        let metric_default = toggler(
            String::from("Use Metric by default"),
            self.data.metric_default,
            Message::MetricDefault,
        )
        .width(Length::Shrink)
        .spacing(10);

        let show_alerts = toggler(
            String::from("Show Alerts"),
            self.data.show_alerts,
            Message::ShowAlerts,
        )
        .width(Length::Shrink)
        .spacing(10);

        let auto_update_internet_resources = toggler(
            String::from("Auto Update Internet Resources"),
            self.data.auto_update_internet_resources,
            Message::AutoUpdateInternetResources,
        )
        .width(Length::Shrink)
        .spacing(10);

        let constant_location = toggler(
            String::from("Constant Location"),
            self.data.constant_location,
            Message::ConstantLocation,
        )
        .width(Length::Shrink)
        .spacing(10);

        let save = button("Save").padding(10).on_press(Message::Save);
        let cancel = button("Cancel")
            .style(iced::theme::Button::Secondary)
            .padding(10)
            .on_press(Message::Cancel);

        let general_column = column![
            row![general]
                .spacing(24)
                .height(50)
                .align_items(Alignment::Center),
            row![layout_file_label, layout_file, pick_layout_file]
                .spacing(10)
                .height(45)
                .align_items(Alignment::Center),
            row![show_alerts]
                .spacing(10)
                .height(40)
                .align_items(Alignment::Center),
            row![auto_update_internet_resources]
                .spacing(10)
                .height(40)
                .align_items(Alignment::Center),
            row![constant_location]
                .spacing(10)
                .height(40)
                .align_items(Alignment::Center),
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);
        let backend_column = column![
            row![backend]
                .spacing(10)
                .height(40)
                .align_items(Alignment::Center),
            row![data_source]
                .spacing(10)
                .height(200)
                .align_items(Alignment::Center),
            row![metric_default]
                .spacing(10)
                .height(40)
                .align_items(Alignment::Center),
            row![api_keys]
                .spacing(10)
                .height(40)
                .align_items(Alignment::Center),
            row![
                openweathermap_api_key_label,
                openweathermap_api_key,
                one_call
            ]
            .spacing(10)
            .height(45)
            .align_items(Alignment::Center),
            row![bing_maps_api_key_label, bing_maps_api_key]
                .spacing(10)
                .height(45)
                .align_items(Alignment::Center)
        ]
        .spacing(20)
        .padding(20)
        .max_width(900);
        let save_cancel = row![cancel, save]
            .spacing(10)
            .height(40)
            .align_items(Alignment::Center);
        let content = column![
            row![general_column, backend_column]
                .spacing(10)
                .align_items(Alignment::Start),
            save_cancel
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
