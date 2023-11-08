use std::fmt;

use dark_light::Mode;
use iced;
use iced::theme::Theme;
use iced::widget::{button, column, container, radio, row, text, text_input, toggler};
use iced::{Alignment, Element, Length, Sandbox};

use local::settings;

pub fn run_settings_app() -> iced::Result {
    App::run(iced::Settings {
        id: None,
        window: iced::window::Settings {
            size: (500, 700),
            position: Default::default(),
            min_size: Some((250, 500)),
            max_size: Some((600, 750)),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
            platform_specific: Default::default(),
        },
        flags: (),
        default_font: None,
        default_text_size: 20.0,
        text_multithreading: true,
        antialiasing: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}

struct App {
    theme: Theme,
    data: settings::Settings,
}

#[derive(Debug, Clone)]
enum Message {
    MetricDefault(bool),
    ShowAlerts(bool),
    AutoUpdateInternetResources(bool),
    EnableDaemon(bool),
    OpenWeatherMapAPIKey(String),
    DataSource(DataSource),
    Cancel,
    Save,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DataSource {
    Meteo,
    OpenWeatherMap,
    OpenWeatherMapOneCall,
    Nws,
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DataSource::Meteo => "meteo".to_string(),
            DataSource::OpenWeatherMap => "openweathermap".to_string(),
            DataSource::OpenWeatherMapOneCall => "openweathermap_onecall".to_string(),
            DataSource::Nws => "nws".to_string(),
        };
        write!(f, "{s}")
    }
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
            Message::Save => self.data.write().unwrap_or(()),
            Message::Cancel => {
                self.data = settings::Settings::new().expect("Loading settings failed")
            }
            Message::MetricDefault(value) => self.data.metric_default = value,
            Message::ShowAlerts(value) => self.data.show_alerts = value,
            Message::AutoUpdateInternetResources(value) => {
                self.data.auto_update_internet_resources = value
            }
            Message::EnableDaemon(value) => self.data.enable_daemon = value,
            Message::OpenWeatherMapAPIKey(value) => self.data.open_weather_map_api_key = value,
            Message::DataSource(value) => {
                self.data.default_backend = value.to_string().to_uppercase()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let data_source = [
            DataSource::Meteo,
            DataSource::OpenWeatherMap,
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
                        "openweathermap_onecall" => DataSource::OpenWeatherMapOneCall,
                        "nws" => DataSource::Nws,
                        _ => DataSource::Meteo,
                    }),
                    Message::DataSource,
                ))
            },
        );
        let openweathermap_api_key_label = text("OpenWeatherMap API key: ");
        let openweathermap_api_key = text_input(
            "OpenWeatherMap API key",
            &self.data.open_weather_map_api_key.clone(),
        )
        .on_input(Message::OpenWeatherMapAPIKey)
        .padding(10)
        .size(20);
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
        let enable_daemon = toggler(
            String::from("Enable Daemon"),
            self.data.enable_daemon,
            Message::EnableDaemon,
        )
        .width(Length::Shrink)
        .spacing(10);

        let save = button("Save").padding(10).on_press(Message::Save);
        let cancel = button("Cancel").padding(10).on_press(Message::Cancel);
        let content = column![
            row![data_source]
                .spacing(10)
                .height(200)
                .align_items(Alignment::Center),
            row![openweathermap_api_key_label, openweathermap_api_key]
                .spacing(10)
                .height(50)
                .align_items(Alignment::Center),
            row![metric_default]
                .spacing(10)
                .height(50)
                .align_items(Alignment::Center),
            row![show_alerts]
                .spacing(10)
                .height(50)
                .align_items(Alignment::Center),
            row![auto_update_internet_resources]
                .spacing(10)
                .height(50)
                .align_items(Alignment::Center),
            row![enable_daemon]
                .spacing(10)
                .height(50)
                .align_items(Alignment::Center),
            row![cancel, save]
                .spacing(10)
                .align_items(Alignment::Center),
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

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
