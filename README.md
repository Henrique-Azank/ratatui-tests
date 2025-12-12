
# Ratatui TUI Testing

A collection of terminal user interfaces built with [ratatui](https://ratatui.rs/) for testing terminal applications without GUIs. The source code for the crate can be found [here](https://github.com/ratatui/ratatui).

## Purpose

Basically to test out the base examples contained in the Crate's documentation and also experiment with the overall capabilities of applications based on TUIs instead of GUIs (which usually have a higher complexity of deployment and cross-OS-development).

I also aim to test out expanding the collection of widgets available and (hopefully) contributing with the open-source framework.

## Project Structure

```text
ratatui-tests/
├── .vscode/
│   ├── extensions.json
│   └── settings.json
├── src/
│   ├── bin/
│   │   ├── counter.rs
│   │   └── hello_ratatui.rs
│   └── lib.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
└── README.md
```

## How to Run

This project contains multiple binary targets. You can run them using cargo:

### Hello Ratatui

A simple "Hello World" example using Ratatui.

```bash
cargo run --bin hello-ratatui
```

### Counter App

An interactive counter application.

```bash
cargo run --bin counter-app
```








