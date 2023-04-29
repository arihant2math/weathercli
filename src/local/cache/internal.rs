use crate::now;

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

fn u8_to_string(i: u8) -> String {
    String::from(i as char)
}

pub fn bytes_to_rows(bytes: Vec<u8>) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut current_date = String::new();
    let mut current_count = 0;
    let mut place = Place::Key;
    for b in bytes {
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
    rows
}

pub fn rows_to_bytes(rows: Vec<Row>) -> Vec<u8> {
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
            while count_now > u32::from(u8::MAX) {
                response.push(u8::MAX);
                count_now -= u32::from(u8::MAX);
            }
            response.push(count_now as u8);
        }
    }
    response.push(28);
    response
}

pub fn get_date_string() -> String {
    now().to_string()
}
