use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::color::*;
use crate::layout::LayoutFile;
use crate::local::dirs::layouts_dir;
use crate::local::settings::Settings;
use crate::util::list_dir;

pub fn install(path: String) -> crate::Result<()> {
    let real_path = PathBuf::from_str(&path).unwrap();
    let file_name = real_path.file_name().ok_or("Not a file")?.to_str().unwrap();
    let ext = real_path.extension().unwrap_or_else(|| "".as_ref());
    if ext != "res" {
        return Err("File has to have an extension of .res")?;
    }
    if file_name == "default.res" {
        return Err("File name cannot be default.res,\
        as it conflicts with the default layout filename,\
        please rename the file and try again.")?; // TODO: Prompt for a new name?
    }
    fs::copy(&real_path, layouts_dir()?.join(file_name))?;
    println!("Checking validity ..."); // TODO: Tech debt (don't copy, check first)
    let test = LayoutFile::new(file_name.to_string());
    match test {
        Err(e) => {
            println!("Invalid layout, {e}");
            fs::remove_file(layouts_dir()?.join(file_name))?;
        },
        Ok(_) => println!("Valid layout!")
    }
    Ok(())
}

pub fn list(settings: Settings) -> crate::Result<()> {
    let paths = fs::read_dir(layouts_dir()?)?;
    let current_layout = settings.internal.layout_file;
    for path in paths {
        let tmp = path?.file_name();
        let file_name = tmp.to_str().unwrap();
        if file_name == current_layout {
            println!("{FORE_MAGENTA}*{FORE_GREEN} {file_name}{RESET}");
        }
        else {
            println!("{FORE_BLUE}  {file_name}");
        }
    }
    Ok(())
}

pub fn select(settings: Settings) -> crate::Result<()> {
    let paths = list_dir(layouts_dir()?)?;
    let current = &*settings.internal.layout_file;
    let current_index = paths.iter().position(|c| c == current).unwrap_or(0); // TODO: make it default.res
    let choice = crate::prompt::choice(&paths, current_index, None)?;
    let mut settings = Settings::new()?; // TODO: Fix excess read 
    settings.internal.layout_file = paths[choice].to_string();
    settings.write()?;
    Ok(())
}

pub fn delete(settings: Settings) -> crate::Result<()> {
    let paths = list_dir(layouts_dir()?)?;
    let current = &*settings.internal.layout_file;
    let current_index = paths.iter().position(|c| c == current).unwrap_or(0); // TODO: make it default.res
    let choice = paths[crate::prompt::choice(&paths, current_index, None)?].to_string();
    fs::remove_file(layouts_dir()?.join(&*choice))?;
    if choice == current {
        println!("Please select a new default layout");
        select(settings)?;
    }
    Ok(())
}
