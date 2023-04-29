mod internal;

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, u128};

use crate::local::cache::internal::{bytes_to_rows, get_date_string, rows_to_bytes, Row};
use crate::{local, now};
use local::dirs::home_dir;

fn get_cache_path() -> PathBuf {
    let mut path = home_dir().expect("expect home dir");
    path.push(".weathercli");
    path.push("f.cache");
    path
}

fn read_bytes_from_file() -> Vec<u8> {
    let mut path = home_dir().expect("expect home dir");
    path.push(".weathercli");
    path.push("f.cache");
    if !path.exists() {
        let mut f = File::create(path.display().to_string()).expect("File Creation Failed");
        let to_write: [u8; 0] = [];
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

/// Reads the value of a key from the cache. This does not update the count value, use `update_hits` to do that
/// Returns None if the key does not exist and returns a string otherwise
pub fn read(key: &str) -> Option<String> {
    let path = get_cache_path();
    if !path.exists() {
        return None;
    }
    let buffer = read_bytes_from_file();
    let rows = bytes_to_rows(buffer);
    Some(rows.into_iter().find(|row| row.key == key)?.value)
}

/// writes the key to the cache, overwriting it if it already exists
pub fn write(key: &str, value: &str) {
    let path = get_cache_path();
    let buffer = read_bytes_from_file();
    let mut rows: Vec<Row> = bytes_to_rows(buffer);
    let key_index = rows.clone().into_iter().position(|row| row.key == key);
    let new_row = Row {
        key: key.to_string(),
        value: value.to_string(),
        date: get_date_string(),
        hits: 0,
    };
    rows.push(new_row);
    let len = rows.len();
    if let Some(index) = key_index {
        rows.swap(index, len - 1);
        rows.pop();
    }
    let new_bytes = rows_to_bytes(rows);
    let mut file = File::options()
        .truncate(true)
        .write(true)
        .open(path.display().to_string())
        .expect("File opening failed");
    file.write_all(&new_bytes).expect("Write Failed");
}

/// Bumps the number of hits to the row, makes it so that the row is less likely to be deleted by the pruner
pub fn update_hits(key: String) -> crate::Result<()> {
    let path = get_cache_path();
    let buffer = read_bytes_from_file();
    let mut rows: Vec<Row> = bytes_to_rows(buffer);
    let key_index = rows
        .clone()
        .into_iter()
        .position(|row| row.key == key)
        .ok_or(format!("Key not found, {key}"))?;
    let key_index_usize = key_index;
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
    let new_bytes = rows_to_bytes(rows);
    let mut file = File::options()
        .truncate(true)
        .write(true)
        .open(path.display().to_string())?;
    file.write_all(&new_bytes)?;
    Ok(())
}

fn calculate_power(row: &Row) -> f64 {
    let offset = now().abs_diff(u128::from_str(&row.date).unwrap_or(u128::MAX)) as f64;
    f64::from(row.hits) / (offset / 86_400_000.0)
}

pub fn prune() -> crate::Result<()> {
    let path = get_cache_path();
    let buffer = read_bytes_from_file();
    let mut rows: Vec<Row> = bytes_to_rows(buffer);
    while rows.len() > 100 {
        let powers: Vec<f64> = rows.iter().map(calculate_power).collect();
        let sort = powers
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map_or(0, |(index, _)| index);
        rows.remove(sort);
    }
    let new_bytes = rows_to_bytes(rows);
    let mut file = File::options()
        .truncate(true)
        .write(true)
        .open(path.display().to_string())?;
    file.write_all(&new_bytes)?;
    Ok(())
}
