use pyo3::prelude::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult, Python};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[pyfunction]
fn get_url(url: String, user_agent: Option<String>) -> String {
    let mut app_user_agent = "weathercli/1".to_string();
    if let Some(user_agent) = user_agent { app_user_agent = user_agent }
    let client = reqwest::blocking::Client::builder().user_agent(app_user_agent).build().expect("");
    client.get(url).send()
        .expect("Url Get failed")
        .text()
        .expect("text expected")
}

#[pyfunction]
pub fn get_urls(urls: Vec<String>) -> Vec<String> {
    let data: Vec<_> = urls
        .par_iter()
        .map(|url| {
            reqwest::blocking::get(url)
                .expect("Url Get failed")
                .text()
                .expect("text expected")
        })
        .collect();

    data
}

pub fn register_networking_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "networking")?;
    child_module.add_function(wrap_pyfunction!(get_url, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(get_urls, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
