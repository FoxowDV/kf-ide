use egui::{
    Key, 
    KeyboardShortcut, 
    Modifiers
};

pub const NEW_FILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::N
);
pub const OPEN_FILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::O
);
pub const SAVE_FILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::S
);
pub const SAVE_ALL: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Modifiers::ALT,
    Key::S
);
pub const SAVE_AS: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Modifiers::SHIFT,
    Key::S
);
pub const CLOSE_FILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Modifiers::SHIFT,
    Key::W
);
pub const EXIT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::ALT,
    Key::X
);
pub const COPY: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::C
);
pub const CUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::X
);
pub const PASTE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::V
);
pub const COMPILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Modifiers::SHIFT,
    Key::B
);
pub const COMPILE_AND_RUN: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::F6
);