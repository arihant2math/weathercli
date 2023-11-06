use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use weather_dirs::weathercli_dir;


const VERSION: u8 = 0;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Place {
    Key,
    Value,
    Date,
    Hits,
}

#[derive(Clone)]
pub struct Row {
    pub key: String,
    pub value: String,
    pub date: String,
    pub hits: u32,
}

pub fn u8_to_string(i: u8) -> String {
    String::from(i as char)
}

pub fn get_path() -> crate::Result<PathBuf> {
    Ok(weathercli_dir()?.join("f.cache"))
}

fn read_from_file() -> crate::Result<Vec<u8>> {
    let path = get_path()?;
    if !path.exists() {
        let mut f = File::create(path.display().to_string())?;
        let to_write: [u8; 0] = [];
        f.write_all(&to_write)?;
    }
    let mut f = File::options()
        .read(true)
        .open(path.display().to_string())?;
    let metadata = fs::metadata(path.display().to_string())?;
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn write_to_file(bytes: &[u8]) -> crate::Result<()> {
    let path = get_path()?;
    let mut file = File::options()
        .truncate(true)
        .write(true)
        .open(path.display().to_string())
        .expect("File opening failed");
    file.write_all(bytes)?;
    Ok(())
}

pub fn read_cache() -> crate::Result<Vec<Row>> {
    let original_bytes = read_from_file()?;
    let version = original_bytes[0];
    if version != VERSION {
        write_cache(Vec::new())?;
        return  Ok(vec![]);
    }
    let bytes = &original_bytes[1..];
    let mut rows: Vec<Row> = Vec::new();
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut current_date = String::new();
    let mut current_count = 0;
    let mut place = Place::Key;
    for &b in bytes {
        if b == 28 {
            rows.push(Row {
                key: current_key,
                value: current_value,
                date: current_date,
                hits: current_count,
            });
            current_key = String::new();
            current_value = String::new();
            current_date = String::new();
            current_count = 0;
            place = Place::Key;
        } else if b == 29 {
            place = Place::Value;
        } else if b == 30 {
            place = Place::Date;
        } else if b == 31 {
            place = Place::Hits;
        } else {
            match place {
                Place::Key => current_key += &*u8_to_string(b),
                Place::Value => current_value += &*u8_to_string(b),
                Place::Date => current_date += &*u8_to_string(b),
                Place::Hits => current_count += u32::from(b),
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
    Ok(rows)
}

pub fn write_cache(rows: Vec<Row>) -> crate::Result<()> {
    let mut response: Vec<u8> = vec![VERSION];
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
            while count_now > u32::from(u8::MAX) {
                response.push(u8::MAX);
                count_now -= u32::from(u8::MAX);
            }
            response.push(count_now as u8);
        }
    }
    response.push(28);
    write_to_file(&response)?;
    Ok(())
}

pub fn get_date_string() -> String {
    crate::now().to_string()
}
