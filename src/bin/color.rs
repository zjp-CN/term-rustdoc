use ratatui::prelude::{Color, Modifier, Style};

pub const BG_CURSOR: Color = Color::Green;
pub const BG_CURSOR_LINE: Color = Color::from_u32(0x0029335b); // #29335b
pub const FG_CURSOR_LINE: Color = Color::from_u32(0x00FFD48E); // #FFD48E

pub const FG_FEATURES: Color = Color::Cyan;
pub const FG_VERSION: Color = Color::from_u32(0x00686363); // #686363

pub const PKG_NAME: Style = Style {
    fg: Some(Color::White),
    add_modifier: Modifier::BOLD,
    ..Style::new()
};

pub const PKG_VERSION: Style = Style {
    fg: Some(FG_VERSION),
    ..Style::new()
};

pub const PKG_FEATURES: Style = Style {
    fg: Some(FG_FEATURES),
    add_modifier: Modifier::ITALIC,
    ..Style::new()
};

// Database Panel
pub const LOADED: Style = Style {
    fg: Some(Color::from_u32(0x00FFD48E)), // #FFD48E
    ..Style::new()
};
pub const CACHED: Style = Style {
    fg: Some(Color::from_u32(0x006FA2FF)), // #6FA2FF
    ..Style::new()
};
pub const HOLDON: Style = Style {
    fg: Some(Color::from_u32(0x00FF768C)), // #FF768C
    ..Style::new()
};

pub const PKG_TOML: Style = Style {
    fg: Some(Color::Green),
    add_modifier: Modifier::BOLD,
    ..Style::new()
};

// Page
pub const HEAD: Style = Style {
    fg: Some(Color::DarkGray),
    bg: Some(Color::LightCyan),
    ..Style::new()
};
pub const SET: Style = Style::new().bg(Color::Rgb(20, 19, 18)); // #141312
pub const NEW: Style = Style::new();
