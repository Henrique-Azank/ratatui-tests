// Third-party dependencies
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

// Function to render the TUI
fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

// Main run loop
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Render the TUI and handle events
        terminal.draw(render)?;

        // Exit on any key press
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

// Main entry point
fn main() -> Result<()> {
    // Instantiate error handling and terminal
    color_eyre::install()?;
    let terminal = ratatui::init();

    // Run the TUI application
    let result = run(terminal);

    // Restore terminal state
    ratatui::restore();

    // Exit with the result of the run loop
    result
}
