use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use deepl::{DeepLApi, Lang};
use ratatui::DefaultTerminal;
use std::{io::Error, time::Duration};

use crate::{
    models::{Mode, Status},
    ui::render_app,
};

pub struct App {
    pub status: Status,
    pub mode: Mode,
    pub source_language: Lang,
    pub target_language: Lang,
    pub source_text: String,
    pub target_text: String,
    pub show_help: bool,
    pub exit: bool,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub lines: Vec<String>,
    pub deepl_api_key: String,
    pub source_language_selected: usize,
    pub target_language_selected: usize,
}

impl App {
    pub fn new() -> Self {
        let key = std::env::var("DEEPL_API_KEY").unwrap();
        //TODO add the possibility to put the key with argument
        // and other options?
        Self {
            status: Status::Main,
            mode: Mode::Normal,
            source_language: Lang::IT,
            target_language: Lang::EN,
            source_text: String::new(),
            target_text: String::new(),
            show_help: false,
            exit: false,
            cursor_row: 0,
            cursor_col: 0,
            lines: vec![" ".into()],
            deepl_api_key: key,
            source_language_selected: 15,
            target_language_selected: 6,
        }
    }
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while !self.exit {
            if event::poll(Duration::from_millis(16))?
                && let CEvent::Key(key) = event::read()?
            {
                self.handle_key_event(key).await;
            }
            terminal.draw(|frame| render_app(frame, self))?;
        }
        Ok(())
    }
    async fn traslate(&mut self) {
        let api = DeepLApi::with(&self.deepl_api_key).new();
        let translated = api
            .translate_text(self.source_text.clone(), Lang::EN)
            .await
            .unwrap();

        let sentences = translated.translations;
        self.target_text.push_str(&sentences[0].text);
    }
    fn copy_to_clipboard(&self) {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        clipboard.set_text(self.target_text.clone()).unwrap();
    }
    fn paste_from_clipboard(&mut self) {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        if let Ok(text) = clipboard.get_text() {
            self.source_text.push_str(&text);
            self.lines = vec![" ".into()];
            // Build lines from source_text so each row becomes an element.
            self.lines = if self.source_text.is_empty() {
                vec!["".into()]
            } else {
                self.source_text
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            };
        }
    }
    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match self.mode {
                Mode::Normal => match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('h') => self.show_help = !self.show_help,
                    KeyCode::Char('i') => self.mode = Mode::Insert,
                    KeyCode::Char('y') => self.copy_to_clipboard(),
                    KeyCode::Char('p') => self.paste_from_clipboard(),
                    KeyCode::Char('s') => self.status = Status::ChooseLang,
                    KeyCode::Char('t') => {
                        self.traslate().await;
                    }
                    _ => {}
                },
                Mode::Insert => match key_event.code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        self.traslate().await;
                    }
                    KeyCode::Char(c) => {
                        if self.lines.is_empty() {
                            self.lines.push(String::new());
                        }
                        // Clamp cursor_row to lines length
                        let row = self.cursor_row.min(self.lines.len() - 1);
                        let line = &mut self.lines[row];
                        // Clamp cursor_col to line length
                        let col = self.cursor_col.min(line.len());
                        line.insert(col, c);
                        self.cursor_col += 1;
                        self.source_text = self.lines.join("\n");
                    }
                    KeyCode::Enter => {
                        if self.lines.is_empty() {
                            self.lines.push(String::new()); //" ".into(
                        }
                        // Clamp cursor_row to lines length
                        let row = self.cursor_row.min(self.lines.len() - 1);
                        let line = &mut self.lines[row];
                        let col = self.cursor_col.min(line.len());
                        if col < line.len() {
                            let new_line = line.split_off(col);
                            self.lines.push(new_line);
                        } else {
                            self.lines.push(String::new());
                        }
                        self.cursor_row += 1;
                        self.cursor_col = 0;
                        self.source_text = self.lines.join("\n");
                    }
                    KeyCode::Left => {
                        // TODO implement method moves, and memorize last position (in up and down)
                        self.cursor_col = self.cursor_col.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        let line_len = self.lines[self.cursor_row].len();
                        if self.cursor_col + 1 < line_len {
                            self.cursor_col += 1;
                        }
                    }
                    KeyCode::Up => {
                        if self.cursor_row > 0 {
                            self.cursor_row -= 1;
                        }
                        let line_len = self.lines[self.cursor_row].len();
                        self.cursor_col = self.cursor_col.min(line_len.saturating_sub(1));
                    }
                    KeyCode::Down => {
                        if self.cursor_row < self.lines.len() - 1 {
                            self.cursor_row += 1;
                        } //TODO add the passage in a empty lines
                        let line_len = self.lines[self.cursor_row].len();
                        self.cursor_col = self.cursor_col.min(line_len.saturating_sub(1));
                    }
                    KeyCode::Backspace => {
                        if self.cursor_col > 0 {
                            self.lines[self.cursor_row].remove(self.cursor_col - 1);
                            self.cursor_col -= 1;
                        } else {
                            if self.cursor_row > 0 {
                                let line = self.lines[self.cursor_row].clone();
                                let new_col_pos = self.lines[self.cursor_row - 1].len();
                                self.lines[self.cursor_row - 1].push_str(&line);
                                self.lines.remove(self.cursor_row);
                                self.cursor_row -= 1;
                                self.cursor_col = new_col_pos;
                            }
                        }
                    }
                    _ => {}
                },
            }
        };
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
