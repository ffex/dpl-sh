use crate::app::App;
use deepl::Lang;
use std::io::Result;

mod app;
mod models;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--help") {
        display_help();
        return Ok(());
    }

    let mut terminal = ratatui::init();
    let mut app = App::new();

    let app_result = app.run(&mut terminal).await;
    ratatui::restore();
    app_result
}

fn display_help() {
    println!("Usage: dpl-sh [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --help     Display this help message and exit");
}

pub fn all_languages() -> &'static [Lang] {
    &[
        Lang::AR,
        Lang::BG,
        Lang::CS,
        Lang::DA,
        Lang::DE,
        Lang::EL,
        Lang::EN,
        Lang::EN_GB,
        Lang::EN_US,
        Lang::ES,
        Lang::ET,
        Lang::FI,
        Lang::FR,
        Lang::HU,
        Lang::ID,
        Lang::IT,
        Lang::JA,
        Lang::KO,
        Lang::LT,
        Lang::LV,
        Lang::NB,
        Lang::NL,
        Lang::PL,
        Lang::PT,
        Lang::PT_BR,
        Lang::PT_PT,
        Lang::RO,
        Lang::RU,
        Lang::SK,
        Lang::SL,
        Lang::SV,
        Lang::TR,
        Lang::UK,
        Lang::ZH,
        Lang::ZH_HANS,
        Lang::ZH_HANT,
    ]
}
