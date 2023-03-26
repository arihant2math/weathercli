use clap::Parser;

#[derive(Clone, Parser)]
struct Cli {
    #[arg(long, short)]
    install_dir: bool,
    #[clap(long, short, action)]
    add_to_path: bool,
    #[clap(long, short, action)]
    guided: bool,
    #[clap(long, short, action)]
    quiet: bool,
}

fn main() {
    let args = Cli::parse();
}