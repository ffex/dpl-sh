use std::{io::Error, time::Duration};

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use crate::{
    models::{Language, LanguageCode, Mode, Status},
    ui::render_app,
};

pub struct App {
    pub status: Status,
    pub mode: Mode,
    pub source_language: &'static Language,
    pub target_language: &'static Language,
    pub source_text: String,
    pub target_text: String,
    pub show_help: bool,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            status: Status::Main,
            mode: Mode::Normal,
            source_language: Language::from_code(LanguageCode::IT),
            target_language: Language::from_code(LanguageCode::EN),
            source_text: String::new(),
            target_text: String::new(),
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
            match self.mode {
                Mode::Normal => match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('h') => self.show_help = !self.show_help,
                    KeyCode::Char('i') => self.mode = Mode::Insert,
                    _ => {}
                },
                Mode::Insert => match key_event.code {
                    KeyCode::Esc => self.mode = Mode::Normal,
                    KeyCode::Char(c) => self.source_text.push(c),
                    _ => {}
                },
            }
        };
    }
}
