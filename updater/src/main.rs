use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    if cfg!(windows) {
        println!("Downloading latest version ...");
        let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/weather.exe").expect("url expected").bytes().expect("bytes expected");
        let mut file = OpenOptions::new().write(true).open("weather.exe").unwrap();
        file.write_all(&data).expect("Something went wrong opening the file");
        println!("Update finished");
    }
    else if  cfg!(unix) {
        let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/weather").expect("url expected").bytes().expect("bytes expected");
        let mut file = OpenOptions::new().write(true).open("weather").unwrap();
        file.write_all(&data).expect("Something went wrong opening the file");
        println!("Update finished");
    }
    else {
        println!("OS unsupported");
    }
}
