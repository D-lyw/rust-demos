pub mod ui;

use std::io;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyboardEnhancementFlags, MouseButton, MouseEvent, MouseEventKind,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Position, Rect, Size},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};
use tui_textarea::{Input, Key, TextArea};
use ui::{create_dialog_block, create_textarea_block};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    ratatui::restore();
    app_result
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DialogState {
    #[default]
    Normal,
    Active,
}
#[derive(Debug)]
pub struct App<'a> {
    exit: bool,
    input: TextArea<'a>,
    selected_dialog: usize,
    dialog_list_state: Vec<DialogState>,
    scroll_offset: Vec<u16>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            exit: false,
            input: create_textarea_block(),
            selected_dialog: 0,
            dialog_list_state: vec![
                DialogState::Normal,
                DialogState::Normal,
                DialogState::Normal,
            ],
            scroll_offset: vec![0, 0, 0],
        }
    }
}

impl<'a> Widget for &'a App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, conversation_area, input_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(5),
        ])
        .areas(area);

        // Render dialog list
        self.render_dialog_list(conversation_area, buf);
        // Render the input area
        self.input.render(input_area, buf);

        let header_title = Line::from("Chat with multi LLMs".bold().style(Style::new().white()));
        Paragraph::new(header_title)
            .bold()
            .centered()
            .render(header_area, buf);
    }
}

impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            if self.exit {
                break;
            }
            terminal.draw(|frame| self.draw(frame))?;
            let viewport = terminal.size()?;
            self.handle_events(viewport)?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self, viewport: Size) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event, viewport),
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let input_event: Input = key_event.into();

        if input_event.key == Key::Enter && input_event.alt == true {
            self.input.insert_newline();
        } else if input_event.key == Key::Enter {
            self.send_chat_message();
        } else if input_event.key == Key::Esc {
            self.exit();
        } else {
            self.input.input(key_event);
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent, viewport: Size) {
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if self.dialog_list_state[self.selected_dialog] == DialogState::Active {
                    self.dialog_list_state[self.selected_dialog] = DialogState::Normal;
                } else {
                    self.dialog_list_state[self.selected_dialog] = DialogState::Active;
                }
            }
            MouseEventKind::ScrollDown => {
                self.scroll_offset[self.selected_dialog] =
                    self.scroll_offset[self.selected_dialog].saturating_add(1);
            }
            MouseEventKind::ScrollUp => {
                self.scroll_offset[self.selected_dialog] =
                    self.scroll_offset[self.selected_dialog].saturating_sub(1);
            }
            MouseEventKind::Moved => {
                let mouse_position = Position::new(mouse_event.column, mouse_event.row);
                let dialog_count = self.dialog_list_state.len();

                for index in 0..dialog_count {
                    let dialog_width = viewport.width / dialog_count as u16;
                    let area = Rect::new(
                        dialog_width * index as u16,
                        0,
                        dialog_width,
                        viewport.height,
                    );
                    if area.contains(mouse_position) {
                        self.selected_dialog = index;
                        self.dialog_list_state[index] = DialogState::Active;
                    } else {
                        self.dialog_list_state[index] = DialogState::Normal;
                    }
                }
            }
            _ => {}
        }
    }

    fn send_chat_message(&mut self) {
        let message = self.input.lines().join("\n");
        self.input.select_all();
        self.input.delete_newline();
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_dialog_list(&self, area: Rect, buf: &mut Buffer) {
        let conversation_list = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(area);

        let mock_dialog = vec![
            Line::from("Hello, how are you doing today?").alignment(Alignment::Left),
            Line::from("I'm doing great, thanks for asking! I'am a ai bot, dsaklfjks akfjds akfjdlskjdskflja ksdljfk sljdkf lsjk l!").alignment(Alignment::Right),
            Line::from("I'm glad to hear that!").alignment(Alignment::Left),
            Line::from("I'm doing great, thanks for asking! I'am a ai bot, dsaklfjks akfjds akfjdlskjdskflja ksdljfk sljdkf lsjk l!").alignment(Alignment::Right),
            Line::from("Hello, how are you doing today?").alignment(Alignment::Left),
            Line::from("I'm doing great, thanks for asking! I'am a ai bot, dsaklfjks akfjds akfjdlskjdskflja ksdljfk sljdkf lsjk l!").alignment(Alignment::Right),
            Line::from("I'm glad to hear that!").alignment(Alignment::Left),
            Line::from("I'm doing great, thanks for asking! I'am a ai bot, dsaklfjks akfjds akfjdlskjdskflja ksdljfk sljdkf lsjk l!").alignment(Alignment::Right),
        ];

        Paragraph::new(mock_dialog.clone())
            .bold()
            .wrap(Wrap { trim: true })
            .block(create_dialog_block("ChatGPT", self.dialog_list_state[0]))
            .centered()
            .scroll((self.scroll_offset[0], 0))
            .render(conversation_list[0], buf);
        Paragraph::new(mock_dialog.clone())
            .bold()
            .wrap(Wrap { trim: true })
            .block(create_dialog_block("Deepseek", self.dialog_list_state[1]))
            .centered()
            .scroll((self.scroll_offset[1], 0))
            .render(conversation_list[1], buf);
        Paragraph::new(mock_dialog)
            .bold()
            .wrap(Wrap { trim: true })
            .block(create_dialog_block("Claude", self.dialog_list_state[2]))
            .centered()
            .scroll((self.scroll_offset[2], 0))
            .render(conversation_list[2], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
