use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct GeocodePointsJSON {
    #[serde(rename = "type")]
    pub r#type: String,
    pub coordinates: Vec<f64>,
    #[serde(rename = "calculationMethod")]
    pub calculation_method: String,
    #[serde(rename = "usageTypes")]
    pub usage_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AddressJSON {
    #[serde(rename = "adminDistrict")]
    pub admin_district: String,
    #[serde(rename = "countryRegion")]
    pub country_region: String,
    #[serde(rename = "formattedAddress")]
    pub formatted_address: String,
    #[serde(rename = "adminDistrict2")]
    pub admin_district2: Option<String>,
    pub locality: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct PointJSON {
    #[serde(rename = "type")]
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ResourceJSON {
    pub __type: String,
    pub bbox: Vec<f64>,
    pub name: String,
    pub point: PointJSON,
    pub address: AddressJSON,
    pub confidence: String,
    #[serde(rename = "entityType")]
    pub entity_type: String,
    #[serde(rename = "geocodePoints")]
    pub geocode_points: Vec<GeocodePointsJSON>,
    #[serde(rename = "matchCodes")]
    pub match_codes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ResourceSetsJSON {
    #[serde(rename = "estimatedTotal")]
    pub estimated_total: i64,
    pub resources: Vec<ResourceJSON>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct BingJSON {
    #[serde(rename = "authenticationResultCode")]
    pub authentication_result_code: String,
    #[serde(rename = "brandLogoUri")]
    pub brand_logo_uri: String,
    pub copyright: String,
    #[serde(rename = "resourceSets")]
    pub resource_sets: Vec<ResourceSetsJSON>,
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    #[serde(rename = "statusDescription")]
    pub status_description: String,
    #[serde(rename = "traceId")]
    pub trace_id: String,
}
