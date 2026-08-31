use crate::models::doc_source::DocSource;
use crate::screens::{Screen, Screens, doc_source_form::DocSourceForm};

use ratatui::DefaultTerminal;

use rusqlite::Connection;

#[derive(Debug)]
pub struct App {
    exit: bool,
    conn: Connection,
    screen: Box<dyn Screen>,
}

impl App {
    pub fn new(conn: Connection) -> Result<Self, Box<dyn std::error::Error>> {
        let app = App {
            exit: false,
            conn,
            screen: Box::new(DocSourceForm::default()),
        };
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
            terminal.draw(|frame| self.screen.draw(frame))?;
            let next_screen_state = self.screen.handle_events()?;
            if next_screen_state == Screens::Exit {
                self.exit = true;
            } else if next_screen_state != self.screen.current() {
                self.screen = self.get_screen(next_screen_state);
            }
        }
        Ok(())
    }

    fn get_screen(&self, screen: Screens) -> Box<dyn Screen> {
        match screen {
            Screens::DocSourceFormScreen => Box::new(DocSourceForm::default()),
            Screens::Exit => Box::new(DocSourceForm::default()),
        }
    }
}
