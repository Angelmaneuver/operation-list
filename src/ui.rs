use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{palette::tailwind::SLATE, Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::config::Item;

pub fn ui(frame: &mut Frame, app: &mut App<Item>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        app.title.to_string(),
        Style::default().fg(Color::Green),
    ))
    .bold()
    .alignment(Alignment::Center)
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut items = Vec::<ListItem>::new();

    for item in app.list.items.iter().clone() {
        items.push(ListItem::new(Line::from(Span::styled(
            item.description.to_string(),
            Style::default().fg(Color::White),
        ))));
    }

    let list = List::new(items)
        .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, chunks[1], &mut app.list.state);

    let footer = Paragraph::new(Line::from(Span::styled(
        "(Enter) to execute. / (q) to quit.",
        Style::default().fg(Color::Red),
    )))
    .alignment(Alignment::Right)
    .block(Block::default());

    frame.render_widget(footer, chunks[2]);
}
