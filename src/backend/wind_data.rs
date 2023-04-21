use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindData {
    pub speed: f64,
    pub heading: i16,
}

impl WindData {
    fn new(speed: f64, heading: i16) -> Self {
        WindData { speed, heading }
    }
}
