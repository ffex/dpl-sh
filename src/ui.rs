use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListDirection, ListItem, Paragraph, Wrap},
};

use crate::models::Status;
use crate::{app::App, models::Mode};

pub fn render_app(frame: &mut Frame, app: &App) {
    // Initialize the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    print_header(frame, chunks[0]);
    match app.status {
        Status::Main => {
            if app.show_help {
                print_textarea_with_help(frame, app, chunks[1], &app.source_language.name);
            } else {
                print_textarea(frame, app, chunks[1], &app.source_language.name);
            }
            print_textarea(frame, app, chunks[2], &app.target_language.name);
        }
        Status::ChooseLang => {
            print_popup(frame, chunks[1]);
        }
    }
    print_footer(frame, app, chunks[3]);
}
pub fn print_header(frame: &mut Frame, area: Rect) {
    // Title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "DeepL in the shell",
        Style::default().fg(Color::Blue),
    ))
    .block(title_block);

    frame.render_widget(title, area);
}
pub fn print_textarea_with_help(frame: &mut Frame, app: &App, area: Rect, language: &str) {
    // Center Screen
    let chunks_center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(area);

    let textarea = vec![Line::default()];
    let textarea_block = Paragraph::new(textarea)
        .block(Block::bordered().title(language))
        .style(Style::default());

    let helparea = vec![Line::default()];
    let helparea_block = Paragraph::new(helparea)
        .block(Block::bordered().title("help"))
        .style(Style::default());

    frame.render_widget(textarea_block, chunks_center[0]);
    frame.render_widget(helparea_block, chunks_center[1]);
}

pub fn print_textarea(frame: &mut Frame, app: &App, area: Rect, language: &str) {
    let textarea = vec![Line::default()];
    let textarea_block = Paragraph::new(textarea)
        .block(Block::bordered().title(language))
        .style(Style::default());

    frame.render_widget(textarea_block, area);
}
pub fn print_footer(frame: &mut Frame, app: &App, area: Rect) {
    let current_navigation_text = vec![
        Span::styled("STATUS BAR", Style::default().fg(Color::Green)),
        Span::styled(" | ", Style::default().fg(Color::White)),
    ];

    let current_nav_key_hint_text = Span::styled(
        match app.mode {
            Mode::Insert => "- INSERT -",
            _ => "- NORMAL -",
        },
        Style::default().fg(Color::Blue),
    );

    let current_nav = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_nav_key_hint =
        Paragraph::new(current_nav_key_hint_text).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    frame.render_widget(current_nav, footer_chunks[0]);
    frame.render_widget(current_nav_key_hint, footer_chunks[1]);
}
pub fn print_popup(frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(40, 50, area);

    //Clear the area
    frame.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title("Confirm")
        .style(Style::default().bg(Color::Blue));

    let exit_text = Text::styled("popup!:w", Style::default().fg(Color::White));

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    frame.render_widget(exit_paragraph, popup_area);
}

fn centered_rect(perc_x: u16, perc_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - perc_y) / 2),
            Constraint::Percentage(perc_y),
            Constraint::Percentage((100 - perc_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - perc_x) / 2),
            Constraint::Percentage(perc_x),
            Constraint::Percentage((100 - perc_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
