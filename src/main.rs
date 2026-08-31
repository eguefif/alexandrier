pub mod app;
pub mod models;

use ratatui;
use rusqlite::Connection;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("./alexandria_db.db")?;
    let mut app = App::new(conn)?;
    ratatui::run(|terminal| app.run(terminal))
}
