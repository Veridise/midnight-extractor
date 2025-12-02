use clap::Parser as _;
use mdnt_extractor::{app::App, cli::Cli};

fn main() -> anyhow::Result<()> {
    App::new(Cli::parse())?.run()
}
