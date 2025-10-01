use crate::app::App;
use std::io::Result;

mod app;
mod models;
mod ui;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--help") {
        display_help();
        return Ok(());
    }

    let mut terminal = ratatui::init();
    let mut app = App::new();

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

fn display_help() {
    println!("Usage: dpl-sh [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --help     Display this help message and exit");
}
