use clap::Args;

#[derive(Clone, Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct GlobalOpts {
    #[arg(
        long,
        short,
        action,
        global = true,
        help = "Print raw json output, useful for debugging"
    )]
    pub json: bool,
    #[arg(
        long,
        short,
        global = true,
        value_enum,
        help = "Which datasource to use, note that openweathermap requires an API key"
    )]
    pub datasource: Option<String>,
    #[arg(
        long,
        short,
        action,
        global = true,
        help = "the output will be in metric"
    )]
    pub metric: bool,
    #[arg(
        long,
        short,
        action,
        global = true,
        help = "the output will be in imperial, overrides --metric"
    )]
    pub imperial: bool,
    #[arg(
        long,
        short,
        action,
        global = true,
        help = "If used, the location will not be gotten from the win32 api, if applicable"
    )]
    pub no_sys_loc: bool,
    #[arg(long, action, global = true, help = "Enables debugging")]
    pub debug: bool,
}
