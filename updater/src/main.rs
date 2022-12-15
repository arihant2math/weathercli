use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    println!("Downloading latest version ...");
    let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/weather.exe").expect("url expected").bytes().expect("bytes expected");
    let mut file = OpenOptions::new().write(true).open("weather.exe").unwrap();
    file.write_all(&data).expect("Something went wrong opening the file");
}
