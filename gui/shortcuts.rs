use eframe::egui::{Context, KeyboardShortcut, Modifiers};
use iris::Actions;

struct ActionKeyboardShortcuts {
    shortcuts: std::collections::HashMap<Actions, KeyboardShortcut>,
}

impl ActionKeyboardShortcuts {
    pub fn new() -> Self {
        let mut shortcuts = std::collections::HashMap::new();

        // Define each shortcut here
        shortcuts.insert(
            Actions::Explain,
            KeyboardShortcut {
                logical_key: eframe::egui::Key::X,
                modifiers: Modifiers::COMMAND,
            },
        );
        shortcuts.insert(
            Actions::Summarize,
            KeyboardShortcut {
                logical_key: eframe::egui::Key::S,
                modifiers: Modifiers::COMMAND,
            },
        );
        shortcuts.insert(
            Actions::Edit,
            KeyboardShortcut {
                logical_key: eframe::egui::Key::E,
                modifiers: Modifiers::COMMAND,
            },
        );
        shortcuts.insert(
            Actions::Ask,
            KeyboardShortcut {
                logical_key: eframe::egui::Key::A,
                modifiers: Modifiers::COMMAND,
            },
        );

        ActionKeyboardShortcuts { shortcuts }
    }

    pub fn get(&self, action: &Actions) -> Option<&KeyboardShortcut> {
        self.shortcuts.get(action)
    }
}

impl Default for ActionKeyboardShortcuts {
    fn default() -> Self {
        ActionKeyboardShortcuts::new()
    }
}

pub fn check_action_shortcuts(ctx: &Context, active_action: &mut Actions) {
    for (action, shortcut) in ActionKeyboardShortcuts::default().shortcuts.iter() {
        if ctx.input_mut(|i| i.consume_shortcut(shortcut)) {
            *active_action = action.clone();
        }
    }
}
