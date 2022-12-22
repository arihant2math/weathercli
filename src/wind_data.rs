use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub struct WindData {
    #[pyo3(get, set)]
    speed: f64,
    #[pyo3(get, set)]
    heading: i16,
}

#[pymethods]
impl WindData {
    #[new]
    fn new(speed: f64, heading: i16) -> Self {
        WindData { speed, heading }
    }
}
