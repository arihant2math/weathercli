mod global;

use clap::{Args, Parser, Subcommand};

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
    #[command(about = "Open the gui settings editor")]
    Settings,
    #[command(about = "Set a config variable via weather config [key] [value]")]
    Config(ConfigOpts),
    #[command(subcommand)]
    Cache(CacheOpts),
    #[command(subcommand)]
    Layout(LayoutOpts),
    #[command(subcommand)]
    CustomBackend(BackendOpts),
    #[command(about = "Run the interactive terminal setup")]
    Setup,
    #[command(about = "Update weathercli")]
    Update(UpdateOpts),
    #[command(about = "Various Credits")]
    Credits,
}

#[derive(Clone, Subcommand)]
pub enum LayoutOpts {
    Install(InstallOpts),
    List,
    Select,
    Delete
}

#[derive(Clone, Subcommand)]
pub enum BackendOpts {
    Install(InstallOpts),
    List,
    Delete
}

#[derive(Clone, Args)]
pub struct InstallOpts {
    pub path: String
}

#[derive(Clone, Subcommand)]
pub enum CacheOpts {
    #[command(about = "Trim the size of the cache")]
    Prune,
    #[command(about = "Delete the cache")]
    Clear,
}

#[derive(Clone, Args)]
pub struct ConfigOpts {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Clone, Args)]
pub struct PlaceOpts {
    pub query: String,
}

#[derive(Clone, Copy, Args)]
pub struct UpdateOpts {
    #[arg(long, short, action, help = "Forces a reinstall of weathercli")]
    pub force: bool,
}
