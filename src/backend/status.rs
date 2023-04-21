use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Status {
    OK = 0,
    ServerError = 1,
    InvalidApiKey = 2,
}
