use clap::Parser;

mod args;

pub fn run() -> color_eyre::Result<()> {
    let args = args::Args::parse();

    Ok(())
}
