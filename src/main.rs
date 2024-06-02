use app::App;

use color_eyre::{
    eyre::Ok,
    Result,
};

mod app;
mod errors;
mod tui;
mod database;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}