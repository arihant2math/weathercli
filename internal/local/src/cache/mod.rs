use std::str::FromStr;
use std::u128;
use std::io;

use thiserror::Error;

use internal::{get_date_string, read_cache, Row, write_cache};

use crate::now;

mod internal;


/// Reads the value of a key from the cache. This does not update the count value, use `update_hits` to do that
/// Returns None if the key does not exist and returns a string otherwise
pub fn read(key: &str) -> Option<String> {
    let rows = read_cache().ok()?; // TODO: Log
    Some(rows
        .into_iter()
        .find(|row| row.key == key)?
        .value)
}

/// writes the key to the cache, overwriting it if it already exists
pub fn write(key: &str, value: &str) -> io::Result<()> {
    let mut rows: Vec<Row> = read_cache()?;
    let key_index = rows.iter().position(|row| row.key == key);
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
    write_cache(rows)?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Row not found: {0}")]
    RowNotFound(usize),
    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error)
}

/// Deletes the key from the cache
pub fn delete(key: &str) -> Result<(), CacheError> {
    let mut rows: Vec<Row> = read_cache()?;
    let key_index = rows
        .iter()
        .position(|row| row.key == key)
        .ok_or(CacheError::KeyNotFound(String::from(key)))?;
    rows.remove(key_index);
    write_cache(rows)?;
    Ok(())
}

/// Bumps the number of hits to the row, makes it so that the row is less likely to be deleted by the pruner
pub fn update_hits(key: &str) -> Result<(), CacheError> {
    let mut rows: Vec<Row> = read_cache()?;
    let key_index = rows
        .iter()
        .position(|row| row.key == key)
        .ok_or(CacheError::KeyNotFound(key.to_string()))?;
    let key_index_usize = key_index;
    let row = rows.get(key_index_usize).ok_or(CacheError::RowNotFound(key_index_usize))?;
    rows.push(Row {
        key: row.key.to_string(),
        value: row.value.to_string(),
        date: get_date_string(),
        hits: row.hits + 1,
    });
    let size = rows.len();
    rows.swap(key_index_usize, size - 1);
    rows.pop();
    write_cache(rows)?;
    Ok(())
}

fn calculate_power(row: &Row) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let offset = now().abs_diff(u128::from_str(&row.date).unwrap_or(u128::MAX)) as f64;
    f64::from(row.hits) / (offset / 86_400_000.0)
}

pub fn prune() -> io::Result<()> {
    let mut rows: Vec<Row> = read_cache()?;
    while rows.len() > 100 {
        let powers: Vec<f64> = rows.iter().map(calculate_power).collect();
        let sort = powers
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map_or(0, |(index, _)| index);
        rows.remove(sort);
    }
    write_cache(rows)?;
    Ok(())
}
