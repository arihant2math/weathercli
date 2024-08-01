/// # Cache v2
/// Trait based caching system
///
/// ## TODO
/// - TODO: Add hit counter and pruning version of hashmap
/// - TODO: Add tests
/// - TODO: general cleanup
/// - TODO: Add docs
/// - TODO: implement cache for more primitive types
use std::collections::HashMap;
use std::fmt::Debug;

use thiserror::Error;

use weather_structs::{Coordinates, LocationData};

use crate::cache_v2;

static CACHE_VERSION: u8 = 2;

#[derive(Debug, Error)]
pub enum CacheItemReadError {
    #[error("unknown value: {0}")]
    UnknownValue(u8),
    #[error("too short: {0}<{1}")]
    TooShort(usize, usize),
    #[error("too long: {0}>{1}")]
    TooLong(usize, usize),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("unsupported version: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum CacheReadError {
    #[error("cache item read error: {0}")]
    ItemError(#[from] CacheItemReadError),
    #[error("unsupported version: {0}")]
    UnsupportedVersion(u8),
    #[error("invalid magic number: {0:?}")]
    InvalidMagicNumber(Vec<u8>),
    #[error("cache item too short: {0}<{1}")]
    TooShort(usize, usize),
    #[error("cache item #{0} too long:")]
    ItemTooLong(usize),
    #[error("cache item #{0} too short:")]
    ItemTooShort(usize),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("other: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum CacheItemWriteError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("other: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum CacheWriteError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("cache item write error: {0}")]
    ItemError(#[from] CacheItemWriteError),
    #[error("other: {0}")]
    Other(String),
}

pub trait Cacheable: Sized {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError>;
    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError>;

    fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self, CacheReadError> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes)?;
        if bytes.is_empty() {
            return Ok(Self::from_cache_bytes(&[0, 10, 10, 0, CACHE_VERSION])?);
        }
        if &bytes[0..3] != &[0, 10, 10, 0] {
            return Err(CacheReadError::InvalidMagicNumber(bytes[0..3].to_vec()));
        }
        if &bytes[4] != &CACHE_VERSION {
            return Err(CacheReadError::Other(format!(
                "unsupported version: {}",
                bytes[0]
            )));
        }
        Ok(Self::from_cache_bytes(&bytes)?)
    }

    fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> Result<(), CacheWriteError> {
        let bytes = self.to_cache_bytes()?;
        writer.write_all(&[0, 10, 10, 0, CACHE_VERSION])?;
        writer.write_all(&bytes)?;
        Ok(())
    }

    fn from_file(path: &std::path::Path) -> Result<Self, CacheReadError> {
        let mut file = std::fs::File::open(path)?;
        Self::from_reader(&mut file)
    }

    // TODO: Better name
    fn to_file(&self, path: &std::path::Path) -> Result<(), CacheWriteError> {
        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        self.to_writer(&mut file)
    }
}

impl Cacheable for () {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(vec![])
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 0 {
            return Err(CacheItemReadError::TooLong(bytes.len(), 0));
        }
        Ok(())
    }
}

impl Cacheable for bool {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(vec![*self as u8])
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 1 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 1));
        }
        Ok(bytes[0] != 0)
    }
}

impl Cacheable for u8 {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(vec![*self])
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 1 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 1));
        }
        Ok(bytes[0])
    }
}

impl Cacheable for i8 {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(vec![*self as u8])
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 1 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 1));
        }
        Ok(bytes[0] as i8)
    }
}

impl Cacheable for u16 {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 2 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 2));
        }
        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl Cacheable for i16 {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 2 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 2));
        }
        Ok(i16::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl Cacheable for u32 {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 4 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 4));
        }
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl Cacheable for i32 {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 4 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 4));
        }
        Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl Cacheable for u64 {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 8 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 8));
        }
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl Cacheable for i64 {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() != 8 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 8));
        }
        Ok(i64::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl Cacheable for String {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        Ok(self.as_bytes().to_vec())
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        Ok(String::from_utf8_lossy(bytes).to_string())
    }
}

impl<T: Cacheable> Cacheable for Vec<T> {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        let len = (self.len() as u32).to_le_bytes();
        let mut bytes = vec![len[0], len[1], len[2], len[3]];
        for item in self.iter() {
            let item_bytes = item.to_cache_bytes()?;
            bytes.extend_from_slice(&(item_bytes.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&item_bytes);
        }
        Ok(bytes)
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Vec<T>, CacheItemReadError> {
        let mut offset = 0;
        if bytes.len() < 4 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 4));
        }

        let len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            if bytes.len() < offset + 4 {
                return Err(CacheItemReadError::TooShort(bytes.len(), offset + 4));
            }
            let item_len =
                u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if bytes.len() < offset + item_len {
                return Err(CacheItemReadError::TooShort(bytes.len(), offset + item_len));
            }
            let item = T::from_cache_bytes(&bytes[offset..offset + item_len])?;
            offset += item_len;
            vec.push(item);
        }
        Ok(vec)
    }
}

impl<T: Cacheable> Cacheable for Option<T> {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        match self {
            Some(item) => Ok(item.to_cache_bytes()?),
            None => Ok(vec![]),
        }
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(T::from_cache_bytes(bytes)?))
        }
    }
}

impl<T: Cacheable, E: Cacheable> Cacheable for Result<T, E> {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        match self {
            Ok(item) => {
                let mut bytes = vec![0];
                bytes.append(&mut item.to_cache_bytes()?);
                Ok(bytes)
            }
            Err(item) => {
                let mut bytes = vec![1];
                bytes.append(&mut item.to_cache_bytes()?);
                Ok(bytes)
            }
        }
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Result<T, E>, CacheItemReadError> {
        if bytes.is_empty() {
            return Err(CacheItemReadError::TooShort(0, 1));
        }
        match bytes[0] {
            0 => Ok(Ok(T::from_cache_bytes(&bytes[1..])?)),
            1 => Ok(Err(E::from_cache_bytes(&bytes[1..])?)),
            _ => Err(CacheItemReadError::UnknownValue(bytes[0])),
        }
    }
}

impl<K: Cacheable + std::hash::Hash + Eq, V: Cacheable> Cacheable for HashMap<K, V> {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        let mut bytes = vec![];
        for (key, value) in self.iter() {
            let key_bytes = key.to_cache_bytes()?;
            let value_bytes = value.to_cache_bytes()?;
            bytes.extend_from_slice(&(key_bytes.len() as u16).to_le_bytes());
            bytes.extend_from_slice(&key_bytes);
            bytes.extend_from_slice(&(value_bytes.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&value_bytes);
        }
        Ok(bytes)
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        let mut offset = 0;
        let mut map = HashMap::default();
        while offset < bytes.len() {
            if bytes.len() < offset + 2 {
                return Err(CacheItemReadError::TooShort(bytes.len(), offset + 2));
            }
            let key_len =
                u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as usize;
            offset += 2;
            if bytes.len() < offset + key_len {
                return Err(CacheItemReadError::TooShort(bytes.len(), offset + key_len));
            }
            let key = K::from_cache_bytes(&bytes[offset..offset + key_len])?;
            offset += key_len;
            if bytes.len() < offset + 4 {
                return Err(CacheItemReadError::TooShort(bytes.len(), offset + 4));
            }
            let value_len =
                u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if bytes.len() < offset + value_len {
                return Err(CacheItemReadError::TooShort(
                    bytes.len(),
                    offset + value_len,
                ));
            }
            let value = V::from_cache_bytes(&bytes[offset..offset + value_len])?;
            offset += value_len;
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl Cacheable for Coordinates {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, CacheItemWriteError> {
        let mut out = self.latitude.to_le_bytes().to_vec();
        out.extend_from_slice(&self.longitude.to_le_bytes().to_vec());
        Ok(out)
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, CacheItemReadError> {
        if bytes.len() < 16 {
            return Err(CacheItemReadError::TooShort(bytes.len(), 8));
        }
        Ok(Self {
            latitude: f64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            longitude: f64::from_le_bytes(bytes[8..16].try_into().unwrap()),
        })
    }
}

impl cache_v2::Cacheable for LocationData {
    fn to_cache_bytes(&self) -> Result<Vec<u8>, crate::cache_v2::CacheItemWriteError> {
        let mut h = HashMap::new();
        h.insert("country".to_string(), self.country.to_string());
        if let Some(city) = &self.city {
            h.insert("city".to_string(), city.to_string());
        }
        if let Some(state) = &self.state {
            h.insert("state".to_string(), state.to_string());
        }
        if let Some(county) = &self.county {
            h.insert("county".to_string(), county.to_string());
        }
        if let Some(village) = &self.village {
            h.insert("village".to_string(), village.to_string());
        }
        if let Some(suburb) = &self.suburb {
            h.insert("suburb".to_string(), suburb.to_string());
        }
        h.to_cache_bytes()
    }

    fn from_cache_bytes(bytes: &[u8]) -> Result<Self, crate::cache_v2::CacheItemReadError> {
        let h: HashMap<String, String> = HashMap::from_cache_bytes(bytes)?;
        Ok(LocationData {
            country: h.get("country").unwrap().to_string(),
            city: h.get("city").map(|s| s.to_string()),
            state: h.get("state").map(|s| s.to_string()),
            county: h.get("county").map(|s| s.to_string()),
            village: h.get("village").map(|s| s.to_string()),
            suburb: h.get("suburb").map(|s| s.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_empty() {
        assert_eq!(Cache::<()>::new(), Cache::<()>::from_bytes(&[]).unwrap());
    }

    #[test]
    fn test_read_empty_bytes() {
        let bytes = vec![0, 10, 10, 0, CACHE_VERSION];
        assert_eq!(Cache::<()>::new(), Cache::<()>::from_bytes(&bytes).unwrap());
    }

    #[test]
    fn test_empty_read_write() {
        let cache = Cache::<String>::new();
        assert_eq!(
            Cache::<String>::from_bytes(&cache.to_bytes().unwrap()).unwrap(),
            cache
        )
    }

    #[test]
    fn test_blank_read_write() {
        let mut cache = Cache::<()>::new();
        cache.insert("key".to_string(), ());
        assert_eq!(
            Cache::<()>::from_bytes(&cache.to_bytes().unwrap()).unwrap(),
            cache
        )
    }

    #[test]
    fn test_string_read_write() {
        let mut cache = Cache::<String>::new();
        cache.insert("key".to_string(), "value".to_string());
        assert_eq!(
            Cache::<String>::from_bytes(&cache.to_bytes().unwrap()).unwrap(),
            cache
        )
    }

    #[test]
    fn test_u64_read_write() {
        let mut cache = Cache::<u64>::new();
        cache.insert("key".to_string(), 123);
        assert_eq!(
            Cache::<u64>::from_bytes(&cache.to_bytes().unwrap()).unwrap(),
            cache
        )
    }

    #[test]
    fn test_multiread_write() {
        let mut cache = Cache::<()>::new();
        cache.insert("key".to_string(), ());
        cache.insert("key2".to_string(), ());
        assert_eq!(
            Cache::<()>::from_bytes(&cache.to_bytes().unwrap()).unwrap(),
            cache
        )
    }
}
