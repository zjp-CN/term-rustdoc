use ratatui::prelude::{Color, Modifier, Style};

/// #29335b
pub const BG_CURSOR_LINE: Color = Color::from_u32(0x0029335b);

pub const FG_FEATURES: Color = Color::Cyan;
/// #686363
pub const FG_VERSION: Color = Color::from_u32(0x00686363);

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
