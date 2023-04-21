use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::cookie::Jar;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Resp {
    pub url: String,
    pub status: u16,
    pub text: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SessionInternalSerialize {
    user_agent: String,
    header_map: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Session {
    client: reqwest::blocking::Client,
    internal_serialize: SessionInternalSerialize,
}

impl Session {
    pub fn new(user_agent: Option<String>, headers: Option<HashMap<String, String>>) -> Self {
        let jar: Jar = Jar::default();
        let app_user_agent = get_user_agent(user_agent);
        let header_map = get_header_map(headers.clone());
        let client = reqwest::blocking::Client::builder()
            .user_agent(&app_user_agent)
            .cookie_store(true)
            .default_headers(header_map)
            .cookie_provider::<Jar>(Arc::new(jar))
            .build()
            .unwrap();
        Session {
            client,
            internal_serialize: SessionInternalSerialize {
                user_agent: app_user_agent,
                header_map: headers.unwrap_or(HashMap::new()),
            },
        }
    }

    pub fn get(&self, url: String) -> Resp {
        let data = self.client.get(url).send().expect("Url Get failed");
        let url = data.url().to_string();
        let status = data.status().as_u16();
        let bytes = data.bytes().unwrap().to_vec();
        let mut text = String::new();
        for byte in bytes.clone() {
            text += &(byte as char).to_string();
        }
        Resp {
            url,
            status,
            text,
            bytes,
        }
    }
}

fn get_header_map(headers: Option<HashMap<String, String>>) -> HeaderMap {
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
    header_map
}

fn get_user_agent(custom: Option<String>) -> String {
    let mut app_user_agent = "weathercli/1".to_string();
    if let Some(user_agent) = custom {
        app_user_agent = user_agent
    }
    app_user_agent
}

/// :param url: the url to retrieve
/// :param user_agent: the user agent to use, weathercli/1 by default
/// :param headers: optional dictionary with headers in it
/// :param cookies: optional list of cookies
pub fn get_url(
    url: String,
    user_agent: Option<String>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
) -> Resp {
    let jar: Jar = Jar::default();
    if let Some(cookies) = cookies {
        let mut formatted_cookies: Vec<String> = Vec::new();
        for (key, value) in cookies {
            formatted_cookies.push(key + "=" + &value);
        }
        for cookie in formatted_cookies {
            jar.add_cookie_str(&cookie, &url.parse::<Url>().unwrap());
        }
    }
    let app_user_agent = get_user_agent(user_agent);
    let header_map = get_header_map(headers);
    let client = reqwest::blocking::Client::builder()
        .user_agent(app_user_agent)
        .cookie_store(true)
        .default_headers(header_map)
        .cookie_provider::<Jar>(Arc::new(jar))
        .build()
        .unwrap();
    let data = client.get(url).send().expect("Url Get failed");
    let url = data.url().to_string();
    let status = data.status().as_u16();
    let bytes = data.bytes().unwrap().to_vec();
    let mut text = String::from("");
    for byte in bytes.clone() {
        text += &(byte as char).to_string();
    }
    Resp {
        url,
        status,
        text,
        bytes,
    }
}

/// Async retrival of multiple urls
/// :param urls: the urls to retrieve
/// :param user_agent: the user agent to use, weathercli/1 by default
/// :param headers: optional dictionary with headers in it
/// :param cookies: optional list of cookies
pub fn get_urls(
    urls: Vec<String>,
    user_agent: Option<String>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
) -> Vec<Resp> {
    let jar: Jar = Jar::default();
    if let Some(cookies) = cookies {
        let mut formatted_cookies: Vec<String> = Vec::new();
        for (key, value) in cookies {
            formatted_cookies.push(key + "=" + &value);
        }
        for cookie in formatted_cookies {
            for url in urls.clone() {
                jar.add_cookie_str(&cookie, &url.parse::<Url>().unwrap());
            }
        }
    }
    let app_user_agent = get_user_agent(user_agent);
    let header_map = get_header_map(headers);
    let client = reqwest::blocking::Client::builder()
        .default_headers(header_map)
        .user_agent(app_user_agent)
        .cookie_store(true)
        .cookie_provider::<Jar>(Arc::new(jar))
        .build()
        .unwrap();
    let data: Vec<_> = urls
        .par_iter()
        .map(|url| {
            let data = client.get(url).send().unwrap();
            let url = data.url().to_string();
            let status = data.status().as_u16();
            let bytes = data.bytes().unwrap().to_vec();
            let mut text = String::from("");
            for byte in bytes.clone() {
                text += &(byte as char).to_string();
            }
            Resp {
                url,
                status,
                text,
                bytes,
            }
        })
        .collect();
    data
}

#[cfg(feature = "python")]
pub fn register_networking_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "networking")?;
    child_module.add_class::<Session>()?;
    child_module.add_function(wrap_pyfunction!(get_url, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(get_urls, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
