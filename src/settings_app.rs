use iced::{Alignment, Element, Length, Sandbox, Settings};
use iced::theme::Theme;
use iced::widget::{button, column, container, radio, row, text, text_input, toggler};

use crate::local::settings::SettingsJson;

pub fn run_settings_app() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    theme: Theme,
    data: SettingsJson
}

#[derive(Debug, Clone)]
enum Message {
    MetricDefault(bool),
    ShowAlerts(bool),
    AutoUpdateInternetResources(bool),
    EnableDaemon(bool),
    OpenWeatherMapAPIKey(String),
    DataSource(DataSource),
    Save,
}

fn save(data: SettingsJson) {
    let mut settings = crate::local::settings::Settings::new();
    settings.internal = data;
    settings.write();
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DataSource {
    Meteo,
    OpenWeatherMap,
    NWS,
    TheWeatherChannel
}

impl ToString for DataSource {
    fn to_string(&self) -> String {
        return match self {
            DataSource::Meteo => "meteo".to_string(),
            DataSource::OpenWeatherMap => "openweathermap".to_string(),
            DataSource::NWS => "nws".to_string(),
            DataSource::TheWeatherChannel => "theweatherchannel".to_string(),
        }
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let mut a = App::default();
        let settings = crate::local::settings::Settings::new();
        a.data = settings.internal;
        a
    }

    fn title(&self) -> String {
        String::from("Settings - WeatherCLI")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Save => save(self.data.clone()),
            Message::MetricDefault(value) => self.data.metric_default = Some(value),
            Message::ShowAlerts(value) => self.data.show_alerts = Some(value),
            Message::AutoUpdateInternetResources(value) => self.data.auto_update_internet_resources = Some(value),
            Message::EnableDaemon(value) => self.data.enable_daemon = Some(value),
            Message::OpenWeatherMapAPIKey(value) => self.data.open_weather_map_api_key = Some(value),
            Message::DataSource(value) => self.data.default_backend = Some(value.to_string()),
        }
    }

    fn view(&self) -> Element<Message> {
        let data_source = [DataSource::Meteo, DataSource::OpenWeatherMap, DataSource::NWS, DataSource::TheWeatherChannel]
                .iter()
                .fold(
                    column![text("Default Backend:")].spacing(10),
                    |column, data_source| {
                        column.push(radio(
                            format!("{data_source:?}"),
                            *data_source,
                            Some(match &*self.data.default_backend.clone().unwrap_or("meteo".to_string()) {
                                "openweathermap" => DataSource::OpenWeatherMap,
                                "nws" => DataSource::NWS,
                                "theweatherchannel" => DataSource::TheWeatherChannel,
                                _ => DataSource::Meteo
                            }),
                            Message::DataSource,
                        ))
                    },
                );
        let openweathermap_api_key_label = text("OpenWeatherMap API key: ");
        let openweathermap_api_key = text_input(
            "OpenWeatherMap API key",
            &self.data.open_weather_map_api_key.clone().unwrap_or("".to_string()),
            Message::OpenWeatherMapAPIKey,
        )
        .padding(10)
        .size(20);
        let metric_default = toggler(
            String::from("Use Metric by default"),
            self.data.metric_default.unwrap_or(true),
            Message::MetricDefault,
        )
        .width(Length::Shrink)
        .spacing(10);
        let show_alerts = toggler(
            String::from("Show Alerts"),
            self.data.show_alerts.unwrap_or(true),
            Message::ShowAlerts,
        )
        .width(Length::Shrink)
        .spacing(10);
        let auto_update_internet_resources = toggler(
            String::from("Auto Update Internet Resources"),
            self.data.auto_update_internet_resources.unwrap_or(true),
            Message::AutoUpdateInternetResources,
        )
        .width(Length::Shrink)
        .spacing(10);
        let enable_daemon = toggler(
            String::from("Enable Daemon"),
            self.data.enable_daemon.unwrap_or(false),
            Message::EnableDaemon,
        )
        .width(Length::Shrink)
        .spacing(10);


        let button = button("Save")
        .padding(10)
        .on_press(Message::Save);

        let content = column![
            row![data_source].spacing(10).height(200).align_items(Alignment::Center),
            row![openweathermap_api_key_label, openweathermap_api_key].spacing(10).height(50).align_items(Alignment::Center),
            row![metric_default].spacing(10).height(50).align_items(Alignment::Center),
            row![show_alerts].spacing(10).height(50).align_items(Alignment::Center),
            row![auto_update_internet_resources].spacing(10).height(50).align_items(Alignment::Center),
            row![enable_daemon].spacing(10).height(50).align_items(Alignment::Center),
            row![button].spacing(10).align_items(Alignment::Center),
        ].spacing(20).padding(20).max_width(600);

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
