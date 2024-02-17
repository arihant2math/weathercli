use local::settings::{SavedLocation, Settings};
#[allow(unused_imports)]
use terminal::color::*;

use crate::arguments::{PlaceOpts, SavedOpts};

fn list(settings: Settings) -> crate::Result<()> {
    let locations = settings.saved_locations;
    for location in locations {
        println!("{FORE_BLUE}  {}", location.name);
    }
    Ok(())
}

pub fn select(settings: Settings) -> crate::Result<SavedLocation> {
    let locations = settings.saved_locations;
    let choices: Vec<String> = locations.iter().map(|l| l.name.clone()).collect();
    let choice = terminal::prompt::radio(&choices, 0, None)?;
    Ok(locations[choice].clone())
}

fn delete(settings: Settings) -> crate::Result<()> {
    let locations = settings.saved_locations;
    let choices: Vec<String> = locations.iter().map(|l| l.name.clone()).collect();
    let choice = terminal::prompt::radio(&choices, 0, None)?;
    let mut settings = Settings::new()?;
    settings.saved_locations.remove(choice);
    settings.write()?;
    Ok(())
}

fn add(opts: PlaceOpts, settings: Settings) -> crate::Result<()> {
    let query = opts.query.unwrap_or(terminal::prompt::input(Some("Search: ".to_string()), None)?);
    let coords = local::location::geocode(query, &settings.bing_maps_api_key)?;
    let name = terminal::prompt::input(Some("Save As: ".to_string()), None)?;
    let mut settings = Settings::new()?;
    settings.saved_locations.push(SavedLocation {
        name,
        latitude: coords.latitude,
        longitude: coords.longitude,
    });
    settings.write()?;
    Ok(())
}

pub fn subcommand(arg: SavedOpts, settings: Settings) -> crate::Result<()> {
    match arg {
        SavedOpts::List => list(settings)?,
        SavedOpts::Delete => delete(settings)?,
        SavedOpts::Add(opts) => add(opts, settings)?
    };
    Ok(())
}
