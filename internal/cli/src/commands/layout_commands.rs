use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use layout::LayoutFile;
use local::list_dir;
use local::settings::Settings;
#[allow(unused_imports)]
use terminal::color::*;
use weather_dirs::layouts_dir;

use crate::arguments::LayoutOpts;

fn install(path: String) -> crate::Result<()> {
    let real_path = PathBuf::from_str(&path).unwrap();
    let mut file_name = real_path
        .file_name()
        .ok_or("Not a file")?
        .to_str()
        .ok_or("to_str failed")?
        .to_string();
    let ext = real_path.extension().unwrap_or_else(|| "".as_ref());
    if ext != "res" {
        return Err(layout::Error::Other(
            "File has to have an extension of .res".to_string(),
        ))?;
    }
    while file_name == "default.res" {
        println!(
            "File name cannot be default.res,\
        as it conflicts with the default layout filename,\
        please rename the file and try again."
        );
        file_name = terminal::prompt::input(Some("Enter a new name: ".to_string()), None)?;
    }
    println!("Checking validity ...");
    let test = LayoutFile::from_path(real_path.to_str().unwrap());
    match test {
        Err(e) => {
            println!("Invalid layout, {e}");
        }
        Ok(_) => {
            println!("Valid layout!");
            fs::copy(&real_path, layouts_dir()?.join(&file_name))?;
        }
    }
    Ok(())
}

fn list(settings: Settings) -> crate::Result<()> {
    let paths = fs::read_dir(layouts_dir()?)?;
    let current_layout = settings.layout_file;
    for path in paths {
        let tmp = path?.file_name();
        let file_name = tmp.to_str().unwrap();
        if file_name == current_layout {
            println!("{FORE_LIGHTMAGENTA}*{FORE_GREEN} {file_name}{RESET}");
        } else {
            println!("{FORE_BLUE}  {file_name}");
        }
    }
    Ok(())
}

fn select(settings: Settings) -> crate::Result<()> {
    let paths = list_dir(layouts_dir()?)?;
    let current = &*settings.layout_file;
    let current_index = paths
        .iter()
        .position(|c| c == current)
        .unwrap_or(paths.iter().position(|c| c == "default.res").unwrap_or(0));
    let choice = terminal::prompt::radio(&paths, current_index, None)?;
    let mut settings = Settings::new()?; // TODO: Fix excess read
    settings.layout_file = paths[choice].to_string();
    settings.write()?;
    Ok(())
}

fn delete(settings: Settings) -> crate::Result<()> {
    let paths = list_dir(layouts_dir()?)?;
    let current = &*settings.layout_file;
    let current_index = paths
        .iter()
        .position(|c| c == current)
        .unwrap_or(paths.iter().position(|c| c == "default.res").unwrap_or(0));
    let choice = paths[terminal::prompt::radio(&paths, current_index, None)?].to_string();
    fs::remove_file(layouts_dir()?.join(&*choice))?;
    if choice == current {
        println!("Please select a new default layout");
        select(settings)?;
    }
    Ok(())
}

fn info(name: String) -> crate::Result<()> {
    // TODO: Add more info
    let paths = fs::read_dir(layouts_dir()?)?;
    for path in paths {
        let tmp = path?.file_name();
        let file_name = tmp.to_str().unwrap();
        if file_name == name.clone() + ".res" {
            println!("{FORE_LIGHTBLUE}==={FORE_LIGHTGREEN} {file_name} {FORE_LIGHTBLUE}==={RESET}");
            let layout = LayoutFile::new(file_name)?;
            println!("Version: {}", layout.version);
            // TODO: print the type of file
        }
    }
    Ok(())
}

pub fn subcommand(arg: LayoutOpts, settings: Settings) -> crate::Result<()> {
    match arg {
        LayoutOpts::Install(opts) => install(opts.path)?,
        LayoutOpts::List => list(settings)?,
        LayoutOpts::Select => select(settings)?,
        LayoutOpts::Delete => delete(settings)?,
        LayoutOpts::Info(opts) => info(opts.name)?,
    };
    Ok(())
}
