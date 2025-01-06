use crossterm::style::Stylize;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tui_textarea::TextArea;

use crate::DialogState;

pub fn create_dialog_block(title: &str, block_state: DialogState) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if block_state == DialogState::Active {
            Color::Red
        } else {
            Color::White
        }))
        .title_style(Style::default().fg(Color::Gray))
}

pub fn create_textarea_block() -> TextArea<'static> {
    let instructions = Line::from(vec![
        Span::raw(" Send "),
        Span::styled("<Enter>", Style::default().fg(Color::Yellow)),
        Span::raw(" New Line "),
        Span::styled("<Enter + Alt>", Style::default().fg(Color::Yellow)),
        Span::raw(" Quit "),
        Span::styled("<ESC> ", Style::default().fg(Color::Yellow)),
    ]);
    let input_block = Block::bordered()
        .title("Input")
        .title_bottom(instructions.centered())
        .border_set(border::ROUNDED);
    let mut input = TextArea::default();
    input.set_block(input_block);

    input
}
