use std::collections::HashMap;
use pyo3::prelude::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult, Python, ToPyObject};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::header::HeaderMap;

#[pyfunction]
fn get_url(url: String, user_agent: Option<String>, headers: Option<HashMap<String, String>>) -> String {
    let mut app_user_agent = "weathercli/1".to_string();
    if let Some(user_agent) = user_agent { app_user_agent = user_agent }
    let client_pre = reqwest::blocking::Client::builder().user_agent(app_user_agent);
    let mut header_map = HeaderMap::new();
    // match headers {
    //     Some(h) => for (k, v) in h.iter() {header_map.insert(k.as_str(), v.parse().unwrap());},
    //     None => ()
    // }
    let client = client_pre.default_headers(header_map).build().expect("");
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
