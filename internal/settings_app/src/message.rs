use crate::datasource::DataSource;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    MetricDefault(bool),
    ShowAlerts(bool),
    ConstantLocation(bool),
    OpenWeatherMapOneCallKey(bool),
    AutoUpdateInternetResources(bool),
    OpenWeatherMapAPIKey(String),
    BingMapsAPIKey(String),
    DataSource(DataSource),
    LayoutFile(String),
    PickLayoutFile,
    Cancel,
    Save,
}