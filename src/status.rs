use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone)]
pub(crate) enum Status {
    OK = 0,
    ServerError = 1,
    InvalidApiKey = 2,
}