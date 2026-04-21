# todo-tui

A terminal UI todo/project manager written in Rust, built on [ratatui](https://ratatui.rs) and [crossterm](https://github.com/crossterm-rs/crossterm). My beginner project in order to learn Rust.

## Status

Early scaffolding. Screen navigation, a project-creation popup, and persisted settings are in place; the per-project todo view and opening an existing project are not yet implemented.

## Run

```sh
cargo run
```

## Capabilities

- Home screen with menu navigation
- Projects screen with Create / Open / Back options
- Centered popup for entering a new project title with live text input
- Settings screen for configuring the base path, persisted to `settings.toml`

## Configuration

On first run, `./settings.toml` is created with defaults:

```toml
base_path = "./"
```

The file is read on every startup. Delete it to regenerate defaults.
