#[derive(Debug)]
pub struct DocSource {
    id: i32,
    name: String,
    config: String,
}

impl DocSource {
    pub fn new(id: i32, name: String, config: String) -> Self {
        Self { id, name, config }
    }

    pub fn create_table_query() -> String {
        return "CREATE TABLE IF NOT EXISTS source (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        config JSONB NOT NULL
        )"
        .to_string();
    }
}
