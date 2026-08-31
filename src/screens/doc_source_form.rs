use crate::models::doc_source::DocSource;
use crate::screens::{Screen, Screens};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct DocSourceForm {
    data: DocSource,
}

impl Screen for DocSourceForm {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn handle_events(&mut self) -> Result<Screens, Box<dyn std::error::Error>> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(Screens::DocSourceFormScreen),
        }
    }

    fn current(&self) -> Screens {
        Screens::DocSourceFormScreen
    }
}

impl DocSourceForm {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<Screens, Box<dyn std::error::Error>> {
        match key_event.code {
            KeyCode::Char('q') => Ok(Screens::Exit),
            _ => Ok(Screens::DocSourceFormScreen),
        }
    }
}

impl Widget for &DocSourceForm {
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
