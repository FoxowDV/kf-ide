use egui::{
    Key, 
    KeyboardShortcut, 
    Modifiers
};

pub const NEW_FILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::N
);
pub const _OPEN_FILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::O
);
pub const _SAVE_FILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::S
);

pub const _SAVE_ALL: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::A
);

pub const _SAVE_AS: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::ALT,
    Key::S
);

pub const _CLOSE_FILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::W
);

pub const _EXIT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::ALT,
    Key::X
);

pub const _COPY: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::C
);

pub const _CUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::X
);

pub const _PASTE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::V
);

pub const _COMPILE: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::B
);

pub const _COMPILE_AND_RUN: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::ALT,
    Key::F6
);
