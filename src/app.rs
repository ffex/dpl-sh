use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use deepl::{DeepLApi, Lang};
use ratatui::{DefaultTerminal, widgets::ListState};
use std::{io::Error, time::Duration};

use crate::{
    all_languages,
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
    pub list_state_source: ListState,
    pub list_state_target: ListState,
    pub is_source_language_selected: bool,
    pub search_text: String,
    pub wait_choose_language: bool,
}

impl App {
    pub fn new() -> Self {
        let key = std::env::var("DEEPL_API_KEY").unwrap();
        //TODO add the possibility to put the key with argument
        // and other options?

        let mut list_state_source = ListState::default();
        list_state_source.select(Some(15));

        let mut list_state_target = ListState::default();
        list_state_target.select(Some(8)); //TODO put this like a constant? a default value?

        Self {
            status: Status::Main,
            mode: Mode::Normal,
            source_language: Lang::IT,
            target_language: Lang::EN_US,
            source_text: String::new(),
            target_text: String::new(),
            show_help: false,
            exit: false,
            cursor_row: 0,
            cursor_col: 0,
            lines: vec![" ".into()],
            deepl_api_key: key,
            list_state_source,
            list_state_target,
            is_source_language_selected: false,
            search_text: String::new(),
            wait_choose_language: false,
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
    async fn traslate(&mut self) -> Option<()>{
        self.target_text.clear();
        if self.source_text.trim().is_empty() {
            return None;
        }
        
        let api = DeepLApi::with(&self.deepl_api_key).new();
        let translated = api
            .translate_text(self.source_text.clone(), self.target_language.clone())
            .source_lang(self.source_language.clone())
            .await
            .unwrap();

        let sentences = translated.translations;
        self.target_text.push_str(&sentences[0].text);
        Some(())
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
        //TODO organize code in methods
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match self.status {
                Status::ChooseLang => {
                    let mut list_state = if self.is_source_language_selected {
                        &mut self.list_state_source
                    } else {
                        &mut self.list_state_target
                    };
                    match key_event.code {
                        KeyCode::Down => list_state.select_next(),
                        KeyCode::Up => list_state.select_previous(),
                        KeyCode::Enter => {
                            self.status = Status::Main;
                            let index = list_state.selected().unwrap();
                            let language = all_languages().get(index).unwrap();
                            if self.is_source_language_selected {
                                self.source_language = language.clone();
                            } else {
                                self.target_language = language.clone();
                            }
                        }
                        KeyCode::Esc => {
                            self.status = Status::Main;
                        }
                        KeyCode::Backspace => {
                            self.search_text.pop();
                        }
                        KeyCode::Char(c) => {
                            self.search_text.push(c);
                            if self.search_text.len() == 2 {
                                let index = all_languages().iter().position(|lang| {
                                    lang.to_string()
                                        .to_lowercase()
                                        .starts_with(&self.search_text.to_lowercase())
                                });
                                if let Some(index) = index {
                                    list_state.select(Some(index));
                                }
                            } else if self.search_text.len() > 2 {
                                let index = all_languages().iter().position(|lang| {
                                    lang.description()
                                        .to_lowercase()
                                        .starts_with(&self.search_text.to_lowercase())
                                });
                                if let Some(index) = index {
                                    list_state.select(Some(index));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Status::Main => match self.mode {
                    Mode::Normal => {
                        if self.wait_choose_language {
                            match key_event.code {
                                KeyCode::Char('t') => {
                                    self.status = Status::ChooseLang;
                                    self.search_text.clear();
                                    self.is_source_language_selected = false;
                                }
                                KeyCode::Char('s') => {
                                    self.status = Status::ChooseLang;
                                    self.search_text.clear();
                                    self.is_source_language_selected = true;
                                }
                                _ => {}
                            }
                            self.wait_choose_language = false;
                        } else {
                            match key_event.code {
                                KeyCode::Char('q') => self.exit = true,
                                KeyCode::Char('?') => self.show_help = !self.show_help,
                                KeyCode::Char('i') => self.mode = Mode::Insert,
                                KeyCode::Char('y') => self.copy_to_clipboard(),
                                KeyCode::Char('p') => self.paste_from_clipboard(),
                                KeyCode::Char('l') => self.wait_choose_language = true,
                                KeyCode::Char('o') => {
                                    self.source_text.clear();
                                    self.target_text.clear();
                                    self.lines = vec![" ".into()];
                                    self.cursor_row = 0;
                                    self.cursor_col = 0;
                                }
                                KeyCode::Char('s') => {
                                    let lang_temp = self.source_language.clone();
                                    self.source_language = self.target_language.clone();
                                    self.target_language = lang_temp;
                                }
                                KeyCode::Char('t') | KeyCode::Enter => {
                                    self.traslate().await;
                                }
                                _ => {}
                            }
                        }
                    }
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
                            self.source_text = self.lines.join("\n");
                        }
                        _ => {}
                    },
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
