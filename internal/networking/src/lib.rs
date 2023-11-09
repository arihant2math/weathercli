use std::collections::HashMap;
use std::io::Read;

use cookie_store::{CookieResult, CookieStore};
use log::trace;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Serialize, Deserialize};

use url::Url;

pub const USER_AGENT: &str = "weathercli/1";
pub const SNEAK_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0";

#[derive(Clone, Serialize, Deserialize)]
pub struct Resp {
    pub status: u16,
    pub bytes: Vec<u8>,
    pub text: String,
}

fn get_user_agent<S: AsRef<str>>(custom: Option<S>) -> String {
    let mut app_user_agent = USER_AGENT.to_string();
    if let Some(user_agent) = custom {
        app_user_agent = user_agent.as_ref().to_string();
    }
    app_user_agent
}

/// get a url
pub fn get_url<S: AsRef<str>>(
    url_s: S,
    user_agent: Option<S>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
) -> std::io::Result<Resp> {
    let url = url_s.as_ref();
    trace!("Retrieving {url}");
    let mut cookies_vec: Vec<CookieResult> = Vec::new();
    for (key, value) in cookies.clone().unwrap_or_default() {
        cookies_vec.push(cookie_store::Cookie::parse(
            key.clone() + "=" + &value,
            &Url::parse(url).expect("parse failed"),
        ));
    }
    let app_user_agent = get_user_agent(user_agent);
    let mut client_pre = ureq::AgentBuilder::new().user_agent(&app_user_agent);
    for (key, value) in headers.unwrap_or_default() {
        client_pre = client_pre.add_header(&key, &value);
    }
    if cookies.is_some() {
        client_pre = client_pre.cookie_store(
            CookieStore::from_cookies(cookies_vec, true).expect("Cookie Store init failed"),
        );
    }
    let client = client_pre.build();
    let req = client.get(url);
    let resp = req.call();
    let real_resp = match resp {
        Ok(d) => Ok(d),
        Err(e) => match e {
            ureq::Error::Status(_s, d) => Ok(d),
            ureq::Error::Transport(d) => Err(d),
        },
    };
    if real_resp.is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionAborted,
            format!("Get to {url} failed"),
        ));
    }
    let data = real_resp.unwrap();
    let status = data.status();
    let mut bytes: Vec<u8> = Vec::with_capacity(100);
    data.into_reader()
        .take(10_000_000)
        .read_to_end(&mut bytes)?;
    let mut text = String::new();
    for byte in &bytes {
        text += &(*byte as char).to_string();
    }
    Ok(Resp {
        status,
        bytes,
        text,
    })
}

/// post to a url
pub fn post_url<S: AsRef<str>>(
    url_s: S,
    data: Option<String>,
    user_agent: Option<S>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
) -> std::io::Result<Resp> {
    let url = url_s.as_ref();
    trace!("Retrieving {url}");
    let mut cookies_vec: Vec<CookieResult> = Vec::new();
    for (key, value) in cookies.clone().unwrap_or_default() {
        cookies_vec.push(cookie_store::Cookie::parse(
            key.clone() + "=" + &value,
            &Url::parse(url).expect("parse failed"),
        ));
    }
    let app_user_agent = get_user_agent(user_agent);
    let mut client_pre = ureq::AgentBuilder::new().user_agent(&app_user_agent);
    for (key, value) in headers.unwrap_or_default() {
        client_pre = client_pre.add_header(&key, &value);
    }
    if cookies.is_some() {
        client_pre = client_pre.cookie_store(
            CookieStore::from_cookies(cookies_vec, true).expect("Cookie Store init failed"),
        );
    }
    let client = client_pre.build();
    let mut real_data = String::new();
    if let Some(ad) = data {
        real_data = ad;
    }
    let req = client.post(url);
    let resp = req.send_string(&real_data);
    let real_resp = match resp {
        Ok(d) => Ok(d),
        Err(e) => match e {
            ureq::Error::Status(_s, d) => Ok(d),
            ureq::Error::Transport(d) => Err(d),
        },
    };
    if real_resp.is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionAborted,
            format!("Get to {url} failed"),
        ));
    }
    let data = real_resp.unwrap();
    let status = data.status();
    let mut bytes: Vec<u8> = Vec::with_capacity(100);
    data.into_reader()
        .take(10_000_000)
        .read_to_end(&mut bytes)?;
    let mut text = String::new();
    for byte in &bytes {
        text += &(*byte as char).to_string();
    }
    Ok(Resp {
        status,
        bytes,
        text,
    })
}

/// Async retrival of multiple urls
/// :param urls: the urls to retrieve
/// :param `user_agent`: the user agent to use, weathercli/1 by default
/// :param headers: optional dictionary with headers in it
/// :param cookies: optional list of cookies
pub fn get_urls(
    urls: &[String],
    user_agent: Option<String>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
) -> std::io::Result<Vec<Resp>> {
    trace!("Retrieving {urls:?}");
    let mut cookies_vec: Vec<CookieResult> = Vec::new();
    for (key, value) in &cookies.clone().unwrap_or_default() {
        for url in urls {
            cookies_vec.push(cookie_store::Cookie::parse(
                key.to_string() + "=" + value,
                &Url::parse(url).expect("parse failed"),
            ));
        }
    }
    let app_user_agent = get_user_agent(user_agent);
    let mut client_pre = ureq::AgentBuilder::new().user_agent(&app_user_agent);
    if cookies.is_some() {
        client_pre = client_pre.cookie_store(
            CookieStore::from_cookies(cookies_vec, true).expect("Cookie Store init failed"),
        );
    }
    for (key, value) in headers.unwrap_or_default() {
        client_pre = client_pre.add_header(&key, &value);
    }
    let client = client_pre.build();
    let data: Vec<_> = urls
        .par_iter()
        .map(|url| {
            let req = client.get(url);
            let data = req.call().expect("Request failed");
            let status = data.status();
            let mut bytes: Vec<u8> = Vec::with_capacity(128);
            data.into_reader()
                .take(u64::MAX - 4)
                .read_to_end(&mut bytes)
                .expect("read failed");

            let mut text = String::new();
            for byte in &bytes {
                text += &(*byte as char).to_string();
            }
            Resp {
                status,
                bytes,
                text,
            }
        })
        .collect();
    Ok(data)
}
