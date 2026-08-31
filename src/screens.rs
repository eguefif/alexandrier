pub mod doc_source_form;

use ratatui::Frame;

#[derive(Debug, PartialEq)]
pub enum Screens {
    DocSourceFormScreen,
    Exit,
}

pub trait Screen: std::fmt::Debug {
    fn draw(&self, frame: &mut Frame);
    fn handle_events(&mut self) -> Result<Screens, Box<dyn std::error::Error>>;
    fn current(&self) -> Screens;
}
