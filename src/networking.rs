use std::collections::HashMap;
use std::str::FromStr;

use pyo3::{pyclass, pyfunction, PyResult, Python, wrap_pyfunction};
use pyo3::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::cookie::Jar;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;

#[pyclass]
#[derive(Clone)]
pub struct Resp {
    #[pyo3(get)]
    url: String,
    #[pyo3(get)]
    status: u16,
    #[pyo3(get)]
    pub(crate) text: String
}

#[pyfunction]
fn get_url(
    url: String,
    user_agent: Option<String>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<Vec<String>>
) -> Resp {
    let jar = Jar::default();
    if let Some(cookies) = cookies {
        for cookie in cookies {
            jar.add_cookie_str(&cookie, &url.parse::<Url>().unwrap());
        }
    }
    let mut app_user_agent = "weathercli/1".to_string();
    if let Some(user_agent) = user_agent {
        app_user_agent = user_agent
    }
    let client_pre = reqwest::blocking::Client::builder().user_agent(app_user_agent);
    let mut header_map = HeaderMap::new();
    let mut heads = HashMap::new();
    if let Some(h) = headers {
        heads = h
    }
    for (k, v) in heads {
        header_map.insert(
            HeaderName::from_str(&k).expect(""),
            HeaderValue::from_str(&v).expect(""),
        );
    }
    let client = client_pre.default_headers(header_map).cookie_store(true).build().unwrap();
    let data = client.get(url).send().expect("Url Get failed");
    Resp {
        url: data.url().to_string(),
        status: data.status().as_u16(),
        text: data.text().expect("Text Expected")
    }
}


#[pyfunction]
pub fn get_urls(urls: Vec<String>) -> Vec<Resp> {
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let data: Vec<_> = urls
        .par_iter()
        .map(|url| {
            let data = client.get(url).send().unwrap();
            Resp {
                url: data.url().to_string(),
                status: data.status().as_u16(),
                text: data.text().expect("Text Expected")
            }
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
