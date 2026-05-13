# todo-tui

A terminal UI todo/project manager written in Rust, built on [ratatui](https://ratatui.rs) and [crossterm](https://github.com/crossterm-rs/crossterm). My beginner project in order to learn Rust.

## Status

Early scaffolding. Projects and notes can be created, listed, opened, and edited; settings are persisted. No richer todo/task model yet.

## Run

```sh
cargo run
```

## Capabilities

- Home screen with menu navigation
- Projects screen with Create / Open / Back options
- Centered popup for entering a new project title with live text input
- Open an existing project from a picker listing directories under the base path
- Notes screen per project with Create / Open / Back options
- Centered popup for entering a new note name, which creates the file on disk
- Open an existing note from a picker, then edit its contents in a multi-line popup editor (Enter inserts a newline, Esc saves and closes)
- Visible cursor in all text-input popups
- Settings screen for configuring the base path, persisted to `settings.toml`

## Configuration

On first run, `./settings.toml` is created with defaults:

```toml
base_path = "./"
```

The file is read on every startup. Delete it to regenerate defaults.
