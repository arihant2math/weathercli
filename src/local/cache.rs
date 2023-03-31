use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use dirs::home_dir;
use pyo3::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Place {
    Key,
    Value,
    Date,
    Hits,
}

#[derive(Clone)]
struct Row {
    key: String,
    value: String,
    date: String,
    hits: i32,
}

pub fn get_date_string() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards :( or there is an overflow error of some sort and stuff broke");
    since_the_epoch.as_millis().to_string()
}

fn u8_to_string(i: u8) -> String {
    String::from(i as char)
}

fn read_bytes_from_file() -> Vec<u8> {
    let mut path = home_dir().expect("expect home dir");
    path.push(".weathercli");
    path.push("f.cache");
    if !path.exists() {
        let mut f = File::create(path.display().to_string()).expect("File Creation Failed");
        let to_write: Vec<u8> = vec![0];
        f.write_all(&to_write).expect("Write failed");
    }
    let mut f = File::options()
        .read(true)
        .open(path.display().to_string())
        .expect("File Open Failed");
    let metadata = fs::metadata(path.display().to_string()).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("Read error");
    buffer
}

/// Reads the value of a key from the cache. This does not update the count value, use update_hits to do that
/// Returns None if the key does not exist and returns a string otherwise
#[pyfunction]
pub fn read(key: String) -> Option<String> {
    let path = get_cache_path();
    if !path.exists() {
        return None;
    }
    let buffer = read_bytes_from_file();
    let mut current_key = "".to_string();
    let mut current_value = "".to_string();
    let mut current_date = "".to_string();
    let mut place = Place::Key;
    for b in buffer {
        if b == 28 {
            if current_key == key {
                return Some(current_value.to_string());
            }
            current_key = String::from("");
            current_value = String::from("");
            current_date = String::from("");
            place = Place::Key;
        } else if b == 29 {
            place = Place::Value
        } else if b == 30 {
            place = Place::Date
        } else if b == 31 {
            place = Place::Hits
        } else {
            match place {
                Place::Key => current_key += &*u8_to_string(b),
                Place::Value => current_value += &*u8_to_string(b),
                Place::Date => current_date += &*u8_to_string(b),
                Place::Hits => (),
            }
        }
    }
    if current_key != *"" && current_key == key {
        return Some(current_value.to_string());
    }
    None
}

fn update_cache(rows: Vec<Row>) -> Vec<u8> {
    let mut response: Vec<u8> = vec![];
    for row in rows {
        if !row.key.is_empty() {
            response.push(28);
            response.append(&mut row.key.into_bytes());
            response.push(29);
            response.append(&mut row.value.into_bytes());
            response.push(30);
            response.append(&mut row.date.into_bytes());
            response.push(31);
            let mut count_now = row.hits;
            while count_now > u8::MAX as i32 {
                response.push(u8::MAX);
                count_now -= u8::MAX as i32;
            }
            response.push(count_now as u8)
        }
    }
    response.push(28);
    response
}

fn get_cache_path() -> PathBuf {
    let mut path = home_dir().expect("expect home dir");
    path.push(".weathercli");
    path.push("f.cache");
    path
}

/// writes the key to the cache, overwriting it if it already exists
#[pyfunction]
pub fn write(key: String, value: String) {
    let path = get_cache_path();
    let buffer = read_bytes_from_file();
    let mut key_index: i64 = -1;
    let mut rows: Vec<Row> = to_rows(buffer);
    for (index, row) in rows.clone().into_iter().enumerate() {
        if row.key == key {
            key_index = index as i64;
        }
    }
    let new_row = Row {
        key,
        value,
        date: get_date_string(),
        hits: 0,
    };
    rows.push(new_row);
    let len = rows.len();
    if key_index != -1 {
        rows.swap(key_index as usize, len - 1);
        rows.pop();
    }
    let new_bytes = update_cache(rows);
    let mut file = File::options()
        .truncate(true)
        .write(true)
        .open(path.display().to_string())
        .expect("File opening failed");
    file.write_all(&new_bytes).expect("Write Failed");
}

fn to_rows(bytes: Vec<u8>) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    let mut current_key = "".to_string();
    let mut current_value = "".to_string();
    let mut current_date = "".to_string();
    let mut current_count = 0;
    let mut place = Place::Key;
    for b in bytes.into_iter() {
        if b == 28 {
            rows.push(Row {
                key: current_key,
                value: current_value,
                date: current_date,
                hits: current_count,
            });
            current_key = "".to_string();
            current_value = "".to_string();
            current_date = "".to_string();
            current_count = 0;
            place = Place::Key;
        } else if b == 29 {
            place = Place::Value
        } else if b == 30 {
            place = Place::Date
        } else if b == 31 {
            place = Place::Hits
        } else {
            match place {
                Place::Key => current_key += &*u8_to_string(b),
                Place::Value => current_value += &*u8_to_string(b),
                Place::Date => current_date += &*u8_to_string(b),
                Place::Hits => current_count += b as i32,
            }
        }
    }
    if current_key != *"" {
        rows.push(Row {
            key: current_key,
            value: current_value,
            date: current_date,
            hits: current_count,
        });
    }
    rows
}

/// Bumps the number of hits to the row, makes it so that the row is less likely to be deleted
#[pyfunction]
pub(crate) fn update_hits(key: String) {
    let path = get_cache_path();
    let buffer = read_bytes_from_file();
    let mut rows: Vec<Row> = to_rows(buffer);
    let mut key_index = -1;
    for (index, row) in rows.clone().into_iter().enumerate() {
        if row.key == key {
            key_index = index as i64;
        }
    }
    if key_index == -1 {
        return;
    }
    let key_index_usize = key_index as usize;
    let row = rows.get(key_index_usize).expect("row not found");
    rows.push(Row {
        key: row.key.to_string(),
        value: row.value.to_string(),
        date: get_date_string(),
        hits: row.hits + 1,
    });
    let size = rows.len();
    rows.swap(key_index_usize, size - 1);
    rows.pop();
    let new_bytes = update_cache(rows);
    let mut file = File::options()
        .truncate(true)
        .write(true)
        .open(path.display().to_string())
        .expect("File opening failed");
    file.write_all(&new_bytes).expect("Write Failed");
}

pub fn register_caching_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "caching")?;
    child_module.add_function(wrap_pyfunction!(read, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(write, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(update_hits, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
