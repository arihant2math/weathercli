#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::io::Write;
use tauri::{LogicalSize, Size};

pub fn path_exists(path: &str) -> bool {
    let chars: Vec<char> = String::from(path).chars().collect();
    if cfg!(windows) {
        if chars[1] != ':' {
            return false
        }
    }
    return true
}

fn get_default_path() -> String {
    return if cfg!(windows) {
        "C:\\weathercli\\".to_string()
    } else {
        "~/.cli/".to_string()
    };
}

#[tauri::command]
fn get_default_path_wrap() -> String {
    get_default_path()
}

fn add_exe_to_path(location: String) {
    if cfg!(windows) {
    }
}

#[tauri::command]
fn install(path: String) -> String {
    let add_to_path = false;
    let mut real_path = path;
    if real_path == "" {
        real_path = get_default_path();
    } else {
        if !path_exists(&real_path) {
            return "Path not Found".to_string();
        }
    }
    let url;
    let location;
    if cfg!(windows) {
        location = real_path.to_string() + "weather.exe";
        url = "https://arihant2math.github.io/weathercli/weather.exe";
    } else {
        location = real_path.to_string() + "weather";
        url = "https://arihant2math.github.io/weathercli/weather";
    }
    let data = reqwest::blocking::get(url).expect("").bytes().expect("");
    fs::create_dir_all(real_path.to_string()).expect("Dir not created");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(location.to_string())
        .unwrap();
    file.write_all(&data)
        .expect("Something went wrong opening the file");
    if add_to_path {
        add_exe_to_path(real_path.to_string())
    }
    let mut config_dir = dirs::home_dir().expect("");
    config_dir.push(".weathercli");
    fs::create_dir_all(config_dir.display().to_string()).expect("Dir not created");
    return "success".to_string();
}

fn main() {
    let t = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![install, get_default_path_wrap]);
    t.run(tauri::generate_context!()).expect("error while running tauri application")
}
