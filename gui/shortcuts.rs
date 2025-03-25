use eframe::egui::{Context, KeyboardShortcut, Modifiers};
use iris::Actions;
use log::debug;

/// A struct to manage keyboard shortcuts for various actions.
pub struct ActionKeyboardShortcuts {
    /// A hashmap mapping Actions to their corresponding KeyboardShortcut.
    shortcuts: std::collections::HashMap<Actions, KeyboardShortcut>,
}

impl ActionKeyboardShortcuts {
    /// Creates a new instance of `ActionKeyboardShortcuts` with predefined shortcuts.
    pub fn new() -> Self {
        let mut shortcuts = std::collections::HashMap::new();

        // Define each shortcut here
        shortcuts.insert(
            Actions::Explain,
            KeyboardShortcut {
                logical_key: eframe::egui::Key::X,
                modifiers: Modifiers::NONE,
            },
        );
        shortcuts.insert(
            Actions::Summarize,
            KeyboardShortcut {
                logical_key: eframe::egui::Key::S,
                modifiers: Modifiers::NONE,
            },
        );
        shortcuts.insert(
            Actions::Edit,
            KeyboardShortcut {
                logical_key: eframe::egui::Key::E,
                modifiers: Modifiers::NONE,
            },
        );
        shortcuts.insert(
            Actions::Ask,
            KeyboardShortcut {
                logical_key: eframe::egui::Key::A,
                modifiers: Modifiers::NONE,
            },
        );

        ActionKeyboardShortcuts { shortcuts }
    }

    /// Retrieves the `KeyboardShortcut` for a given action.
    pub fn get(&self, action: &Actions) -> Option<&KeyboardShortcut> {
        self.shortcuts.get(action)
    }

    /// Checks if any of the defined shortcuts have been pressed and updates the active action accordingly.
    pub fn check(&self, ctx: &Context, active_action: &mut Actions) {
        for (action, shortcut) in self.shortcuts.iter() {
            if ctx.input_mut(|i| i.consume_shortcut(shortcut)) {
                *active_action = *action;
            }
        }
    }
}

impl Default for ActionKeyboardShortcuts {
    /// Provides a default instance of `ActionKeyboardShortcuts` using the `new` method.
    fn default() -> Self {
        ActionKeyboardShortcuts::new()
    }
}

/// A struct to manage the keyboard shortcut for submitting an action.
pub struct SubmitKeyboardShortcut {
    /// The keyboard shortcut for submission.
    shortcut: KeyboardShortcut,
}

impl SubmitKeyboardShortcut {
    /// Checks if the submission shortcut has been pressed.
    pub fn check(&self, ctx: &Context) -> bool {
        ctx.input_mut(|i| i.consume_shortcut(&self.shortcut))
    }
}

impl Default for SubmitKeyboardShortcut {
    /// Provides a default instance of `SubmitKeyboardShortcut` with predefined shortcut (S + COMMAND).
    fn default() -> Self {
        SubmitKeyboardShortcut {
            shortcut: KeyboardShortcut {
                logical_key: eframe::egui::Key::S,
                modifiers: Modifiers::COMMAND,
            },
        }
    }
}

/// A struct to manage keyboard shortcuts for response actions.
pub struct ResponseKeyboardShortcuts {
    /// The keyboard shortcut for copying a response.
    copy: KeyboardShortcut,
}

impl ResponseKeyboardShortcuts {
    /// Checks if the copy shortcut has been pressed and copies the provided response text.
    pub fn check(&self, ctx: &Context, response: String) {
        if ctx.input_mut(|i| i.consume_shortcut(&self.copy)) {
            debug!("Detected Copy event");
            ctx.copy_text(response)
        }
    }
}

impl Default for ResponseKeyboardShortcuts {
    /// Provides a default instance of `ResponseKeyboardShortcuts` with predefined shortcut (C).
    fn default() -> Self {
        ResponseKeyboardShortcuts {
            copy: KeyboardShortcut {
                logical_key: eframe::egui::Key::C,
                modifiers: Modifiers::NONE,
            },
        }
    }
}

