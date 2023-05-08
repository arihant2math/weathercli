use crate::cli::arguments::LayoutOpts;
use ansi::{FORE_BLUE, FORE_GREEN, FORE_LIGHTMAGENTA, RESET};
use crate::layout::LayoutFile;
use crate::local::dirs::layouts_dir;
use crate::local::settings::Settings;
use crate::util::list_dir;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn install(path: String) -> crate::Result<()> {
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
        }
        Ok(_) => println!("Valid layout!"),
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
    let current_index = paths.iter().position(|c| c == current).unwrap_or(0); // TODO: make it default.res
    let choice = crate::prompt::radio(&paths, current_index, None)?;
    let mut settings = Settings::new()?; // TODO: Fix excess read
    settings.layout_file = paths[choice].to_string();
    settings.write()?;
    Ok(())
}

fn delete(settings: Settings) -> crate::Result<()> {
    let paths = list_dir(layouts_dir()?)?;
    let current = &*settings.layout_file;
    let current_index = paths.iter().position(|c| c == current).unwrap_or(0); // TODO: make it default.res
    let choice = paths[crate::prompt::radio(&paths, current_index, None)?].to_string();
    fs::remove_file(layouts_dir()?.join(&*choice))?;
    if choice == current {
        println!("Please select a new default layout");
        select(settings)?;
    }
    Ok(())
}

pub fn subcommand(arg: LayoutOpts, settings: Settings) -> crate::Result<()> {
    match arg {
        LayoutOpts::Install(opts) => install(opts.path)?,
        LayoutOpts::List => list(settings)?,
        LayoutOpts::Select => select(settings)?,
        LayoutOpts::Delete => delete(settings)?,
    };
    Ok(())
}
