use std::collections::HashMap;
use std::io::Read;

use cookie_store::{CookieResult, CookieStore};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use ureq;
use url::Url;

#[derive(Clone, Serialize, Deserialize)]
pub struct Resp {
    pub status: u16,
    pub bytes: Vec<u8>,
    pub text: String,
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
pub fn get_url<S: AsRef<str>>(
    url_s: S,
    user_agent: Option<String>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
) -> crate::Result<Resp> {
    let url = url_s.as_ref();
    let mut cookies_vec: Vec<CookieResult> = Vec::new();
    if cookies.is_some() {
        for (key, value) in cookies.clone().unwrap() {
            cookies_vec.push(cookie_store::Cookie::parse(
                key.clone() + "=" + &value,
                &Url::parse(url).expect("parse failed"),
            ));
        }
    }
    let app_user_agent = get_user_agent(user_agent);
    let mut client_pre = ureq::AgentBuilder::new().user_agent(&app_user_agent);
    for (key, value) in headers.unwrap_or(HashMap::new()) {
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
        Ok(d) =>  Ok(d),
        Err(e) => match e {
            ureq::Error::Status(s, d) => Ok(d),
            ureq::Error::Transport(d) => Err(d)
        }
    };
    if real_resp.is_err() {
        return Err(crate::util::Error::NetworkError(format!("Get to {} failed", url)))
    }
    let data = real_resp.unwrap();
    let status = data.status();
    let mut bytes: Vec<u8> = Vec::with_capacity(100);
    data.into_reader()
        .take(10_000_000)
        .read_to_end(&mut bytes)
        .map(|e| "read failed".to_string());
    let mut text = String::from("");
    for byte in bytes.clone() {
        text += &(byte as char).to_string();
    }
    Ok(Resp {
        status,
        bytes,
        text,
    })
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
) -> crate::Result<Vec<Resp>> {
    let mut cookies_vec: Vec<CookieResult> = Vec::new();
    if cookies.is_some() {
        for (key, value) in cookies.clone().unwrap() {
            for url in &urls {
                cookies_vec.push(cookie_store::Cookie::parse(
                    key.clone() + "=" + &value,
                    &url::Url::parse(url).expect("parse failed"),
                ));
            }
        }
    }
    let app_user_agent = get_user_agent(user_agent);
    let mut client_pre = ureq::AgentBuilder::new().user_agent(&app_user_agent);
    if cookies.is_some() {
        client_pre = client_pre.cookie_store(
            CookieStore::from_cookies(cookies_vec, true).expect("Cookie Store init failed"),
        );
    }
    for (key, value) in headers.unwrap_or(HashMap::new()) {
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
                .take(10_000_000)
                .read_to_end(&mut bytes)
                .expect("read failed");

            let mut text = String::from("");
            for byte in bytes.clone() {
                text += &(byte as char).to_string();
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
