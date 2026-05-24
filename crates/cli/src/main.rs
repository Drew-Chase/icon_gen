
#[tokio::main]
async fn main()->color_eyre::Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();
    icon_gen_cli_lib::run().await
}
