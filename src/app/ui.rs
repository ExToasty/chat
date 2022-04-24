use tui::{
    backend::{Backend},
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, BorderType},
    Frame,
};

use crate::app::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    // TODO: Handle different sizes

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Percentage(10)].as_ref())
        .split(size);

    let title = draw_title();
    f.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(8), Constraint::Length(32)].as_ref())
        .split(chunks[1]);

    let body = draw_body(&app);
    f.render_widget(body, body_chunks[0]);

    let users = draw_users();
    f.render_widget(users, body_chunks[1]);

    let input = draw_input(&app);
    f.render_widget(input, chunks[2]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Insert Cool Chat Client Name")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
}

#[allow(unused_variables)]
fn draw_body(app: &App) -> Paragraph {
    Paragraph::new("TODO: Show messages here.") // TODO: Show message vector in body
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded),
        )
}

fn draw_users<'a>() -> Paragraph<'a>{
    Paragraph::new("Users\nTODO: Need to add users") // TODO: Need to add users
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
}

fn draw_input(app: &App) -> Paragraph {
    Paragraph::new(app.input.as_ref())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
}