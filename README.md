# todo-tui

A terminal UI todo/project manager written in Rust, built on [ratatui](https://ratatui.rs) and [crossterm](https://github.com/crossterm-rs/crossterm). My beginner project in order to learn Rust.

## Status

Early scaffolding. Screen navigation and a project-creation popup are in place; persistence and the per-project todo view are not yet implemented.

## Run

```sh
cargo run
```

## Capabilities

- Home screen with menu navigation
- Projects screen with Create / Open / Back options
- Centered popup for entering a new project title with live text input
