/**
 * Generally, the main file is kept minimal to just handle the entrypoint
 * and the initial setup of the application. The core logic and state management
 * are encapsulated within the `app` module, while the `ui` module is responsible
 * for rendering the user interface and handling user interactions.
 *
 * Some event handling can also be placed here, but generally, it's best to keep
 * this file focused on high-level orchestration rather than detailed logic.
 */
// Standard library dependencies
use std::{error::Error, io};

// Third-party (Ratatui) dependencies
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

// Current module imports
mod app;
mod ui;

// Application runner function
use app::App;

/// Function to run the application, handling the main event loop.
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    // Implement our own event/ui loop
    loop {}
}

/// Main entrypoint for the JSON binary
fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;

    // Create a Stderr handler for the application
    let mut stderr = io::stderr();

    // Enter the alternate screen and enable mouse capture
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    // Instantiate the terminal backend and terminal
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create the application instance
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal to its previous state
    disable_raw_mode()?;

    // Leave the alternate screen and disable mouse capture
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    // Show the cursor again
    terminal.show_cursor()?;

    // Handle any errors from running the application
    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
