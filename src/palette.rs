use ratatui::style::Color;

// Subset of CodeWhale palette constants needed for UI styling.
// Deep Sea (WHALE) dark theme.
pub const WHALE_BG: Color = Color::Rgb(3, 7, 13);
pub const WHALE_PANEL: Color = Color::Rgb(14, 23, 41);
pub const WHALE_TEXT_BODY: Color = Color::Rgb(246, 242, 232);
pub const WHALE_TEXT_DIM: Color = Color::Rgb(105, 119, 145);
pub const WHALE_TEXT_HINT: Color = Color::Rgb(132, 145, 170);
pub const WHALE_ACCENT_PRIMARY: Color = Color::Rgb(106, 174, 242); // action blue
pub const WHALE_ACCENT_SECONDARY: Color = Color::Rgb(79, 209, 197); // seafoam
pub const WHALE_HUMAN: Color = Color::Rgb(246, 196, 83); // gold
pub const WHALE_ERROR: Color = Color::Rgb(255, 134, 178);
pub const WHALE_WARNING: Color = Color::Rgb(255, 122, 89);
pub const WHALE_SUCCESS: Color = Color::Rgb(155, 214, 111);
pub const WHALE_INFO: Color = WHALE_ACCENT_PRIMARY;

// Alias for convenience
pub const BG: Color = WHALE_BG;
pub const PANEL: Color = WHALE_PANEL;
pub const FG: Color = WHALE_TEXT_BODY;
pub const DIM: Color = WHALE_TEXT_DIM;
pub const ACCENT: Color = WHALE_ACCENT_PRIMARY;
pub const SECONDARY: Color = WHALE_ACCENT_SECONDARY;
pub const HIGHLIGHT: Color = WHALE_HUMAN;
pub const ERROR: Color = WHALE_ERROR;
pub const WARNING: Color = WHALE_WARNING;
pub const SUCCESS: Color = WHALE_SUCCESS;
pub const INFO: Color = WHALE_INFO;
