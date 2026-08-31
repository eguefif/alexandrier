# Alex

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- [crossterm](https://crates.io/crates/crossterm) 0.29.0 — terminal input/output handling
- [ratatui](https://crates.io/crates/ratatui) 0.30.2 — terminal UI framework

## Install

```sh
git clone <repo-url>
cd alex
cargo build --release
```

Run it with:

```sh
cargo run
```

## TODO

Currently working on:

- [ ] Add widget form to DocSourceForm
- [ ] Add insert function in DocSource model

### Ingestion

We will work first on two documentation:
- [ ] FastAPI
- [ ] SqlAlchemy

### Source edit

- [ ] Add SQLITE db with a single table: source
- [ ] Configure the table: name, url, ingestion_method
- [ ] Add TUI form
- [ ] Add TUI index
- [ ] Add TUI show

### Search

- [ ] Add TUI interface
    - [ ] search 
    - [ ] result 
    - [ ] overview
