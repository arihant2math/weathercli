use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
pub struct NWSPointGeometry {
    #[serde(rename = "type")]
    pub geo_type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NWSPointProperties {
    #[serde(rename = "@id")]
    pub id: String,
    pub cwa: String,
    pub grid_x: i32,
    pub grid_y: i32,
    pub forecast_grid_data: String,
}

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
pub struct NWSPointJSON {
    pub geometry: NWSPointGeometry,
    pub properties: NWSPointProperties,
}

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NWSValueIntJSON {
    pub valid_time: String,
    pub value: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NWSValueFloatJSON {
    pub valid_time: String,
    pub value: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
pub struct NWSIntDataJSON {
    pub uom: String,
    pub values: Vec<NWSValueIntJSON>,
}

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
pub struct NWSFloatDataJSON {
    pub uom: String,
    pub values: Vec<NWSValueFloatJSON>,
}

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NWSPropertiesJSON {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub _type: String,
    pub update_time: String,
    pub valid_times: String,
    pub forecast_office: String,
    pub grid_id: String,
    pub grid_x: String,
    pub grid_y: String,
    pub temperature: NWSFloatDataJSON,
    pub dewpoint: NWSFloatDataJSON,
    pub max_temperature: NWSFloatDataJSON,
    pub min_temperature: NWSFloatDataJSON,
    pub relative_humidity: NWSFloatDataJSON,
    pub apparent_temperature: NWSFloatDataJSON,
    pub heat_index: NWSFloatDataJSON,
    pub wind_chill: NWSFloatDataJSON,
    pub sky_cover: NWSIntDataJSON,
    pub wind_direction: NWSIntDataJSON,
    pub wind_speed: NWSFloatDataJSON,
    pub wind_gust: NWSFloatDataJSON,
    pub probability_of_precipitation: NWSFloatDataJSON,
    pub quantitative_precipitation: NWSFloatDataJSON,
    pub ice_accumulation: NWSFloatDataJSON,
    pub snowfall_amount: NWSFloatDataJSON,
    pub snow_level: NWSFloatDataJSON,
    pub transport_wind_speed: NWSFloatDataJSON,
    pub transport_wind_direction: NWSFloatDataJSON,
    pub mixing_height: NWSFloatDataJSON,
    pub twenty_foot_wind_speed: NWSFloatDataJSON,
    pub twenty_foot_wind_direction: NWSFloatDataJSON,
    pub wave_height: NWSFloatDataJSON,
}

#[derive(Clone, Serialize, Deserialize ,Debug, PartialEq)]
pub struct NWSJSON {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub properties: NWSPropertiesJSON,
}
