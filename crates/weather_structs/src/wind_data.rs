use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct WindData {
    pub speed: f64,
    pub heading: u16,
}
