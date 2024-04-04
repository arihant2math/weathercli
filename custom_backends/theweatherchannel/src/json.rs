use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WeatherChannelDalSunV3CurrentObservationsUrlConfigJson {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WeatherChannelDalSunV3DailyForecastWithHeadersUrlConfigJson {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WeatherChannelDalJson {
    get_sun_v3_current_observations_url_config:
        HashMap<String, HashMap<String, WeatherChannelDalSunV3CurrentObservationsUrlConfigJson>>,
    get_sun_v3_daily_forecast_with_headers_url_config: HashMap<
        String,
        HashMap<String, WeatherChannelDalSunV3DailyForecastWithHeadersUrlConfigJson>,
    >,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherChannelMainJson {
    dal: WeatherChannelDalJson,
}
