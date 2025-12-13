/**
 * This module defines the core application state and logic for the JSON editor binary.
 * It includes the main `App` struct that holds the state of the application,
 * as well as enums to represent the current screen and editing state.
 *
 * The `App` struct contains fields for the key and value inputs, a HashMap
 * to store the key-value pairs, and the current screen and editing state.
 * It also provides methods to manipulate the state, such as saving key-value pairs,
 * toggling the editing state, and printing the JSON representation of the data.
 */
// Base dependencies
use std::collections::HashMap;

// Enum representing the current screen of the application
#[derive(Debug, PartialEq, Eq)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

// Attribute being edited in the JSON when editing
#[derive(Debug, PartialEq, Eq)]
/// What aspect of the JSON is being edited
pub enum CurrentlyEditing {
    Key,
    Value,
}

// The main application state for the JSON binary
#[derive(Debug)]
/// Main application state for the JSON editor
pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

// Implementing helper methods for the App struct
impl App {
    /// Constructs a new App instance with default values.
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    /// Saves the current key-value pair into the pairs HashMap and resets inputs.
    pub fn save_key_value(&mut self) {
        // Insert the current key-value pair into the HashMap
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        // Clear the input fields and reset editing state
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    /// Toggles the currently editing state between Key and Value.
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    /// Prints the current JSON representation of the key-value pairs.
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }
}
