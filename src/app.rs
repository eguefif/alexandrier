use crate::models::doc_source::DocSource;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use rusqlite::Connection;

#[derive(Debug)]
pub struct App {
    exit: bool,
    conn: Connection,
}

impl App {
    pub fn new(conn: Connection) -> Result<Self, Box<dyn std::error::Error>> {
        let app = App { exit: false, conn };
        app.init()?;
        Ok(app)
    }

    fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let create_source_table_query = DocSource::create_table_query();
        self.conn.execute(&create_source_table_query, ())?;
        Ok(())
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Alexandria Library ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);
        let text = Text::from(vec![Line::from(vec!["value".into()])]);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
