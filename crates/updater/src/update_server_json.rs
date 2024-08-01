use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Index {
    version: String,
    daemon_version: String,
    updater_version: String,
    default_layout_hash: String,
    weather_ascii_images_hash: String,
    weather_codes_hash: String,
    weather_exe_hash_unix: String,
    weather_exe_hash_windows: String,
}
