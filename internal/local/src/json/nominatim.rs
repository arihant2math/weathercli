use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct NominatimJSON {
    pub place_id: i64,
    pub licence: String,
    pub osm_type: String,
    pub osm_id: i64,
    pub lat: String,
    pub lon: String,
    pub category: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub place_rank: i64,
    pub importance: f64,
    pub addresstype: String,
    pub name: String,
    pub display_name: String,
    pub boundingbox: Vec<String>,
}
