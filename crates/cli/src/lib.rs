use clap::Parser;

mod args;

pub async fn run() -> color_eyre::Result<()> {
    let args = args::Args::parse();

    Ok(())
}
