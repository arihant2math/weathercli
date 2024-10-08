use cookie_store::{CookieResult, CookieStore};
use log::trace;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use ureq::{Agent, Response};
use url::Url;

use std::error::Error as ErrorTrait;

pub const USER_AGENT: &str = "weathercli/1";
pub const SNEAK_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0";

#[derive(Debug)]
pub struct Error(io::Error);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error(e)
    }
}

impl ErrorTrait for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Resp {
    pub status: u16,
    pub bytes: Vec<u8>,
    pub text: String,
    pub headers: HashMap<String, String>,
}

impl Resp {
    pub fn new(value: Response) -> Result<Self> {
        let mut headers = HashMap::new();
        for header in value.headers_names() {
            headers.insert(
                header.to_string(),
                value.header(&header).unwrap().to_string(),
            );
        }
        let status = value.status();
        let mut bytes: Vec<u8> = Vec::with_capacity(256);
        value
            .into_reader()
            .take(u64::MAX - 4)
            .read_to_end(&mut bytes)?;
        let mut text = String::new();
        for byte in &bytes {
            text += &(*byte as char).to_string();
        }

        Ok(Resp {
            status,
            bytes,
            text,
            headers,
        })
    }
}

fn get_user_agent<S: AsRef<str>>(custom: Option<S>) -> String {
    let mut app_user_agent = USER_AGENT.to_string();
    if let Some(user_agent) = custom {
        app_user_agent = user_agent.as_ref().to_string();
    }
    app_user_agent
}

fn get_client<S: AsRef<str>>(
    urls: &[String],
    user_agent: Option<S>,
    headers: Option<HashMap<String, String>>,
    cookies: &Option<HashMap<String, String>>,
) -> Agent {
    let mut cookies_vec: Vec<CookieResult> = Vec::new();
    for (key, value) in &cookies.clone().unwrap_or_default().clone() {
        for url in urls {
            cookies_vec.push(cookie_store::Cookie::parse(
                format!("{key}={value}"),
                &Url::parse(url).unwrap_or_else(|_| panic!("url parse failed: {url}")),
            ));
        }
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
    client_pre.build()
}

/// get a url, with the ability the user agent, headers, and cookies
/// If you are faking the user agent, use `SNEAK_USER_AGENT`, otherwise this defaults to `USER_AGENT`
pub fn get_url<S: AsRef<str>>(
    url_s: S,
    user_agent: Option<S>,
    headers: Option<HashMap<String, String>>,
    cookies: &Option<HashMap<String, String>>,
) -> Result<Resp> {
    let url = url_s.as_ref();
    trace!("Retrieving {url}");
    let client = get_client(&[url.to_string()], user_agent, headers, cookies);
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
        ))?;
    }
    let data = real_resp.unwrap();
    Resp::new(data)
}

/// post to a url
pub fn post_url<S: AsRef<str>>(
    url_s: S,
    data: Option<String>,
    user_agent: Option<S>,
    headers: Option<HashMap<String, String>>,
    cookies: &Option<HashMap<String, String>>,
) -> Result<Resp> {
    let url = url_s.as_ref();
    trace!("Retrieving {url}");
    let client = get_client(&[url.to_string()], user_agent, headers, cookies);
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
        ))?;
    }
    let data = real_resp.unwrap();
    Resp::new(data)
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
    cookies: &Option<HashMap<String, String>>,
) -> Result<Vec<Resp>> {
    trace!("Retrieving {urls:?}");
    let client = get_client(&urls.to_vec().clone(), user_agent, headers, cookies);
    let data: Vec<_> = urls
        .par_iter()
        .map(|url| {
            let req = client.get(url);
            let data_r = req.call();
            let data = if let Err(e) = data_r {
                match e {
                    ureq::Error::Status(_s, d) => d,
                    ureq::Error::Transport(d) => panic!("Request failed: {d}"),
                }
            } else {
                data_r.unwrap()
            };

            Resp::new(data).unwrap()
        })
        .collect();
    Ok(data)
}
