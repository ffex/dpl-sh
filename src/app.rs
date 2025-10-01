use std::{io::Error, time::Duration};

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use crate::{
    models::{Language, LanguageCode, Status},
    ui::render_app,
};

pub struct App {
    pub status: Status,
    pub source_language: &'static Language,
    pub target_language: &'static Language,
    pub show_help: bool,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            status: Status::Main,
            source_language: Language::from_code(LanguageCode::IT),
            target_language: Language::from_code(LanguageCode::EN),
            show_help: false,
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while !self.exit {
            // Your code here
            if event::poll(Duration::from_millis(16))?
                && let CEvent::Key(key) = event::read()?
            {
                self.handle_key_event(key);
            }
            terminal.draw(|frame| render_app(frame, self))?;
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            if let KeyCode::Char('q') = key_event.code {
                self.exit = true
            };
        };
    }
}
