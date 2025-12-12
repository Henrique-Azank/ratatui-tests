// Standard library dependencies
use std::io;

// Third-party dependencies
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

// Define the main Application structure
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

// Implement the rendering logic for the application by implementing the Widget trait
impl Widget for &App {
    // Implement the only required method of the Widget trait - render

    /// Renders the TUI components onto the provided buffer
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Define the title and instructions lines
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        // Define a bordered block with title and instructions
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        // Create the counter display text
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            // Use the App State counter value, styled in yellow
            self.counter.to_string().yellow(),
        ])]);

        // Render the paragraph widget with the counter value
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

// Implement the application logic
impl App {
    /// Main TUI drawing function
    fn draw(&self, frame: &mut Frame) {
        // Since the App struct implements Widget, we can render it directly
        frame.render_widget(self, frame.area());
    }

    /// Sets the exit flag to true to terminate the application
    fn exit(&mut self) {
        self.exit = true;
    }

    /// Increments the counter value by 1
    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    /// Decrements the counter value by 1
    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    /// Main function for handling the event logic
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    /// Handles user input events
    fn handle_events(&mut self) -> io::Result<()> {
        // event::read() is a blocking call that waits for the next event

        // Read the next event
        match event::read()? {
            // It's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        // Finish successfully
        Ok(())
    }

    /// Runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // While the application is not set to exit
        while !self.exit {
            // Draw the current state of the application using the terminal
            // You should only call the draw method once for each pass through your application’s main loop.
            terminal.draw(|frame| self.draw(frame))?;

            // Handle user input events
            self.handle_events()?;
        }

        // Finish successfully
        Ok(())
    }
}

// Main binary entry point
fn main() -> io::Result<()> {
    // Instantiate the default terminal
    let mut terminal = ratatui::init();

    // Run the TUI application based on the App struct
    let app_result = App::default().run(&mut terminal);

    // Restore terminal state
    ratatui::restore();

    // Exit with the result of the run loop
    app_result
}
