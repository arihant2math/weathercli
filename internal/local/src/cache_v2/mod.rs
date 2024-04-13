use std::fmt::Debug;
use std::fs;
 use std::path::Path;

use thiserror::Error;

#[derive (Debug, Error)]
pub enum CacheItemReadError {
    #[error("Not enough bytes, expected {0}, got {1}")]
    NotEnoughBytes(usize, usize),
    #[error("Other: {0}")]
    Other(String)
}

#[derive(Debug, Error)]
pub enum CacheReadError {
    #[error("Invalid Version, expected {0}, got {1}")]
    InvalidVersion(u8, u8),
    #[error("CacheItemReadError: {0}")]
    CacheItemReadError(#[from] CacheItemReadError),
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Other: {0}")]
    Other(String)
}

#[derive(Debug, Error)]
pub enum CacheItemWriteError {
    #[error("Other: {0}")]
    Other(String)
}

#[derive(Debug, Error)]
pub enum CacheWriteError {
    #[error("CacheItemWriteError: {0}")]
    CacheItemWriteError(#[from] CacheItemWriteError),
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Other: {0}")]
    Other(String)
}

pub static VERSION: u8 = 0;

pub static LOCATION_CACHE_FILE: &str = "loc.cache";


pub trait Cacheable where Self: Sized {
    fn to_cache_bytes(&self) -> Result<(Vec<u8>, usize), CacheItemWriteError>;
    fn from_cache_bytes(bytes: &[u8]) -> Result<(Self, usize), CacheItemReadError>;
}

#[derive(Debug)]
pub struct Row {
    pub metadata: [u8; 32],
    pub key: String,
    pub value: String,
    pub hits: u32,
}

impl Row {
    pub fn new_entry(key: String, value: String) -> Self {
        Self {
            metadata: [0; 32], // TODO: implement metadata
            key,
            value,
            hits: 0,
        }
    }

    pub fn add_hit(&mut self) {
        self.hits += 1;
    }
}

impl Cacheable for Row {
    fn to_cache_bytes(&self) -> Result<(Vec<u8>, usize), CacheItemWriteError> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.metadata.to_vec());
        bytes.append(&mut self.key.len().to_le_bytes().to_vec());
        bytes.append(&mut self.value.len().to_le_bytes().to_vec());
        bytes.append(&mut self.key.into_bytes());
        bytes.append(&mut self.value.into_bytes());
        bytes.append(&mut self.hits.to_le_bytes().to_vec());
        Ok((bytes, 32 + 4 + 4 + self.key.len() + self.value.len() + 16 + 4))
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<(Self, usize), CacheItemReadError> {
        if bytes.len() < 40 {
            return Err(CacheItemReadError::NotEnoughBytes(40, bytes.len()));
        }
        let mut offset = 0;
        let metadata = bytes[offset..offset + 32].try_into().unwrap();
        offset += 32;
        let key_len = usize::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let value_len = usize::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        if bytes.len() < offset + key_len + value_len + 16 + 4 {
            return Err(CacheItemReadError::NotEnoughBytes(offset + key_len + value_len + 16 + 4, bytes.len()));
        }
        let key = String::from_utf8(bytes[offset..offset + key_len as usize].to_vec()).unwrap();
        offset += key_len;
        let value = String::from_utf8(bytes[offset..offset + value_len as usize].to_vec()).unwrap();
        offset += value_len;
        let hits = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        Ok((Self {
            metadata,
            key,
            value,
            hits,
        }, offset))
    }
}


// TODO: implement Iterator, Debug (when possible)
#[derive(Default)]
pub struct Cache<T: Cacheable> {
    pub rows: Vec<T>,
}

impl <T: Cacheable> Cache<T> {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, row: T) {
        self.rows.push(row);
    }

    pub fn read(buf: Vec<u8>) -> Result<Self, CacheReadError> {
        // TODO: check version
        if buf[0] != VERSION {
            return Err(CacheReadError::InvalidVersion(buf[0], VERSION));
        }
        let buf = &buf[1..];
        let mut rows = Vec::new();
        let mut offset = 0;
        while offset < buf.len() {
            let (row, new_offset) = T::from_cache_bytes(&buf[offset..])?;
            rows.push(row);
            offset += new_offset;
        }
        Ok(Self { rows })
    }

    pub fn write(&self) -> Result<Vec<u8>, CacheWriteError> {
        let mut bytes = Vec::new();
        bytes.push(VERSION);
        for row in &self.rows {
            bytes.append(&mut row.to_cache_bytes()?.0);
        }
        Ok(bytes)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), CacheWriteError> {
        fs::write(path, &self.write()?)?;
        Ok(())
    }
}

// TODO: Fix
// impl <P: AsRef<Path>, T: Cacheable> TryFrom<P> for Cache<T> {
impl <T: Cacheable> Cache<T> {
    // type Error = CacheReadError;

    fn try_from<P: AsRef<Path>>(path: P) -> Result<Self, CacheReadError> {
        let bytes = std::fs::read(path)?;
        Self::read(bytes)
    }
}

impl <T: Cacheable> Clone for Cache<T> where T: Clone {
    fn clone(&self) -> Self {
        Self {
            rows: self.rows.clone(),
        }
    }
}

impl Cache<Row> {
    pub fn get(&self, key: &str) -> Option<&Row> {
        self.rows.iter().find(|row| row.key == key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Row> {
        self.rows.iter_mut().find(|row| row.key == key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Row> {
        Some(self.rows.remove(self.rows.iter().position(|row| row.key == key)?))
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn clear(&mut self) {
        self.rows.clear()
    }
}
