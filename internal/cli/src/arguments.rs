use clap::{Args, Parser, Subcommand};

// TODO: Arguments to add
// - warnings and downloads in weather config
mod global;

#[derive(Clone, Parser)]
#[command(version, author, about, name = "weathercli")]
pub struct App {
    #[command(flatten)]
    pub global_opts: global::GlobalOpts,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    #[command(about = "Get the weather for a specific place")]
    Place(PlaceOpts),
    #[command(subcommand)]
    Saved(SavedOpts),
    #[command(subcommand)]
    Settings(SettingsOpts),
    #[command(subcommand)]
    Cache(CacheOpts),
    #[command(subcommand)]
    Layout(LayoutOpts),
    #[command(subcommand)]
    Backend(BackendOpts),
    #[command(about = "Run the interactive terminal setup")]
    Setup,
    #[command(about = "Update weathercli")]
    Update(UpdateOpts),
    #[command(about = "About weathercli")]
    About,
    #[command(about = "Various Credits")]
    Credits,
}

#[derive(Clone, Subcommand)]
pub enum SavedOpts {
    #[command(about = "Add a saved place")]
    Add(PlaceOpts),
    #[command(about = "List all saved places")]
    List,
    #[command(about = "Delete a saved place")]
    Delete,
}

#[derive(Clone, Subcommand)]
pub enum LayoutOpts {
    #[command(about = "Install a layout")]
    Install(InstallOpts),
    List,
    Select,
    Info(InfoOpts),
    Delete,
}

#[derive(Clone, Subcommand)]
pub enum BackendOpts {
    #[command(about = "Install a custom backend")]
    Install(InstallOpts),
    #[command(about = "List all installed backends")]
    List,
    #[command(about = "Select the default backend")]
    Select,
    #[command(about = "Set the openweathermap API key")]
    OpenWeatherMapApiKey,
    #[command(about = "Set the bing maps API key")]
    BingMapsApiKey,
    #[command(about = "Delete an installed custom backend")]
    Delete,
}

#[derive(Clone, Args)]
pub struct InstallOpts {
    pub path: String,
}

#[derive(Clone, Args)]
pub struct InfoOpts {
    pub name: String,
}

#[derive(Clone, Subcommand)]
pub enum CacheOpts {
    #[command(about = "Info about the cache")]
    Info,
    #[command(about = "Trim the size of the cache")]
    Prune,
    #[command(about = "Delete the cache")]
    Clear,
}

#[derive(Clone, Args)]
pub struct SettingsKeyOpts {
    pub key: Option<String>,
}

#[derive(Clone, Args)]
pub struct SettingsKeyValueOpts {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Subcommand)]
pub enum SettingsOpts {
    #[command(about = "View settings")]
    View(SettingsKeyOpts),
    #[command(about = "Open the settings editor")]
    Edit(SettingsKeyValueOpts),
    #[command(about = "Open the gui settings editor")]
    GuiEdit,
}

#[derive(Clone, Args)]
pub struct PlaceOpts {
    pub query: Option<String>,
}

#[derive(Clone, Copy, Args)]
pub struct UpdateOpts {
    #[arg(long, short, action, help = "Forces a reinstall of weathercli")]
    pub force: bool,
    #[arg(long, action, help = "Dry run the update")]
    pub dry_run: bool,
}
