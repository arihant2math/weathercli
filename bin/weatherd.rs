use std::{thread, time};
use std::env::current_exe;

use auto_launch::AutoLaunchBuilder;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn register() {
    let path = current_exe().unwrap().display().to_string();
    let auto = AutoLaunchBuilder::new()
    .set_app_name("weatherd")
    .set_app_path(&path)
    .set_use_launch_agent(true)
    .build()
    .unwrap();
    if !auto.is_enabled().unwrap() {
        auto.enable().unwrap();
    }
}

fn main() {
    register();
    let sleep_duration = time::Duration::from_secs(60);
    loop {
        println!("Updating Data ...");
        let w = core::weather_file::WeatherFile::new("downloader_urls.txt".to_string());
        let urls_split = w.data.split('\n');
        let urls = urls_split.collect::<Vec<&str>>();
        let data: Vec<_> = urls
            .par_iter()
            .map(|url| {
                reqwest::blocking::get(url.to_string())
                    .expect("Url Get failed").text().unwrap()
            })
            .collect();
        let mut out = core::weather_file::WeatherFile::new("d.cache".to_string());
        let joined = core::cache::get_date_string() + "\n\n\n\n\n"+ &*data.join("\n\n\n\n\n");
        out.data = joined;
        thread::sleep(sleep_duration);
    }
}
