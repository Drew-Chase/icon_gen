#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();
use color_eyre::{Result, install};

fn main() -> Result<()> {
    install()?;
    if std::env::args().len() > 1 {
        // This means that we are running the app from the command line
        icon_gen_cli_lib::run()?;
        return Ok(());
    }

    let main_window = MainWindow::new()?;
    Ok(main_window.run()?)
}
