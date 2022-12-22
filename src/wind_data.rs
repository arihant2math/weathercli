use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub struct WindData {
    speed: f64,
    heading: i16,
}

#[pymethods]
impl WindData {
    #[new]
    fn new(speed: f64, heading: i16) -> Self {
        WindData { speed, heading }
    }

    #[getter(speed)]
    fn speed(&self) -> PyResult<f64> {
        Ok(self.speed)
    }

    #[getter(heading)]
    fn heading(&self) -> PyResult<i16> {
        Ok(self.heading)
    }
}
