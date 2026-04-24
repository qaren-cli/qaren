//! Design system and theme constants for the Qaren GUI.
//!
//! Defines the visual identity from the PRD: color palette, diff
//! highlight colours, and helper functions for applying the theme
//! to the egui context.

use egui::Color32;

// ─────────────────────────────────────────────────────────────────────
// Primary Palette (from PRD §3)
// ─────────────────────────────────────────────────────────────────────

/// Action Blue — primary interaction and branding colour (#3B82F6)
pub const ACTION_BLUE: Color32 = Color32::from_rgb(59, 130, 246);

/// Action Blue hover state — slightly lighter
pub const ACTION_BLUE_HOVER: Color32 = Color32::from_rgb(96, 165, 250);

/// Action Blue pressed/active state — slightly darker
pub const ACTION_BLUE_ACTIVE: Color32 = Color32::from_rgb(37, 99, 235);

// ─────────────────────────────────────────────────────────────────────
// Light Mode Palette
// ─────────────────────────────────────────────────────────────────────

/// Light mode background — off-white
pub const LIGHT_BG: Color32 = Color32::from_rgb(249, 250, 251);

/// Light mode panel/card background — white
pub const LIGHT_PANEL_BG: Color32 = Color32::from_rgb(255, 255, 255);

/// Light mode text — near-black
pub const LIGHT_TEXT: Color32 = Color32::from_rgb(17, 24, 39);

/// Light mode secondary text — gray
pub const LIGHT_TEXT_SECONDARY: Color32 = Color32::from_rgb(107, 114, 128);

/// Light mode border
pub const LIGHT_BORDER: Color32 = Color32::from_rgb(229, 231, 235);

/// Light mode line number gutter
pub const LIGHT_GUTTER_BG: Color32 = Color32::from_rgb(243, 244, 246);

// ─────────────────────────────────────────────────────────────────────
// Dark Mode Palette
// ─────────────────────────────────────────────────────────────────────

/// Dark mode background
pub const DARK_BG: Color32 = Color32::from_rgb(17, 24, 39);

/// Dark mode panel/card background
pub const DARK_PANEL_BG: Color32 = Color32::from_rgb(31, 41, 55);

/// Dark mode text — off-white
pub const DARK_TEXT: Color32 = Color32::from_rgb(243, 244, 246);

/// Dark mode secondary text
pub const DARK_TEXT_SECONDARY: Color32 = Color32::from_rgb(156, 163, 175);

/// Dark mode border
pub const DARK_BORDER: Color32 = Color32::from_rgb(55, 65, 81);

/// Dark mode line number gutter
pub const DARK_GUTTER_BG: Color32 = Color32::from_rgb(31, 41, 55);

// ─────────────────────────────────────────────────────────────────────
// Diff Highlight Colors — Light Mode
// ─────────────────────────────────────────────────────────────────────

/// Added key/value background — green tint
pub const ADDED_BG_LIGHT: Color32 = Color32::from_rgb(220, 252, 231);
/// Added key/value text
pub const ADDED_TEXT_LIGHT: Color32 = Color32::from_rgb(22, 101, 52);

/// Deleted key/value background — red tint
pub const DELETED_BG_LIGHT: Color32 = Color32::from_rgb(254, 226, 226);
/// Deleted key/value text
pub const DELETED_TEXT_LIGHT: Color32 = Color32::from_rgb(153, 27, 27);

/// Modified key/value background — yellow/amber tint
pub const MODIFIED_BG_LIGHT: Color32 = Color32::from_rgb(254, 249, 195);
/// Modified key/value text
pub const MODIFIED_TEXT_LIGHT: Color32 = Color32::from_rgb(133, 77, 14);

/// Identical key/value — dimmed
pub const IDENTICAL_TEXT_LIGHT: Color32 = Color32::from_rgb(156, 163, 175);

// ─────────────────────────────────────────────────────────────────────
// Diff Highlight Colors — Dark Mode
// ─────────────────────────────────────────────────────────────────────

/// Added key/value background — dark green tint
pub const ADDED_BG_DARK: Color32 = Color32::from_rgb(20, 83, 45);
/// Added key/value text — light green
pub const ADDED_TEXT_DARK: Color32 = Color32::from_rgb(187, 247, 208);

/// Deleted key/value background — dark red tint
pub const DELETED_BG_DARK: Color32 = Color32::from_rgb(127, 29, 29);
/// Deleted key/value text — light red
pub const DELETED_TEXT_DARK: Color32 = Color32::from_rgb(254, 202, 202);

/// Modified key/value background — dark amber tint
pub const MODIFIED_BG_DARK: Color32 = Color32::from_rgb(120, 53, 15);
/// Modified key/value text — light amber
pub const MODIFIED_TEXT_DARK: Color32 = Color32::from_rgb(253, 230, 138);

/// Identical key/value — dimmed
pub const IDENTICAL_TEXT_DARK: Color32 = Color32::from_rgb(107, 114, 128);

// ─────────────────────────────────────────────────────────────────────
// Status / Feedback Colors
// ─────────────────────────────────────────────────────────────────────

/// Success green
pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);

/// Warning amber
pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11);

/// Error red
pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);

// ─────────────────────────────────────────────────────────────────────
// Theme helpers
// ─────────────────────────────────────────────────────────────────────

/// Colour set resolved for the current mode (light or dark).
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub bg: Color32,
    pub panel_bg: Color32,
    pub text: Color32,
    pub text_secondary: Color32,
    pub border: Color32,
    pub gutter_bg: Color32,
    pub added_bg: Color32,
    pub added_text: Color32,
    pub deleted_bg: Color32,
    pub deleted_text: Color32,
    pub modified_bg: Color32,
    pub modified_text: Color32,
    pub identical_text: Color32,
}

impl ThemeColors {
    /// Resolve the full colour set for light or dark mode.
    pub fn for_mode(dark_mode: bool) -> Self {
        if dark_mode {
            Self {
                bg: DARK_BG,
                panel_bg: DARK_PANEL_BG,
                text: DARK_TEXT,
                text_secondary: DARK_TEXT_SECONDARY,
                border: DARK_BORDER,
                gutter_bg: DARK_GUTTER_BG,
                added_bg: ADDED_BG_DARK,
                added_text: ADDED_TEXT_DARK,
                deleted_bg: DELETED_BG_DARK,
                deleted_text: DELETED_TEXT_DARK,
                modified_bg: MODIFIED_BG_DARK,
                modified_text: MODIFIED_TEXT_DARK,
                identical_text: IDENTICAL_TEXT_DARK,
            }
        } else {
            Self {
                bg: LIGHT_BG,
                panel_bg: LIGHT_PANEL_BG,
                text: LIGHT_TEXT,
                text_secondary: LIGHT_TEXT_SECONDARY,
                border: LIGHT_BORDER,
                gutter_bg: LIGHT_GUTTER_BG,
                added_bg: ADDED_BG_LIGHT,
                added_text: ADDED_TEXT_LIGHT,
                deleted_bg: DELETED_BG_LIGHT,
                deleted_text: DELETED_TEXT_LIGHT,
                modified_bg: MODIFIED_BG_LIGHT,
                modified_text: MODIFIED_TEXT_LIGHT,
                identical_text: IDENTICAL_TEXT_LIGHT,
            }
        }
    }
}

/// Apply the Qaren theme (light or dark) to an egui context.
///
/// Configures the global visuals (background, text color, widget rounding,
/// selection color) to match the PRD design system.
pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
    let mut visuals = if dark_mode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    let colors = ThemeColors::for_mode(dark_mode);

    // Background
    visuals.panel_fill = colors.panel_bg;
    visuals.window_fill = colors.panel_bg;
    visuals.extreme_bg_color = colors.bg;

    // Selection
    visuals.selection.bg_fill = ACTION_BLUE.linear_multiply(0.3);
    visuals.selection.stroke = egui::Stroke::new(1.0, ACTION_BLUE);

    // Hyperlink
    visuals.hyperlink_color = ACTION_BLUE;

    // Widget rounding — slightly rounded for modern feel
    visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
    visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
    visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
    visuals.widgets.active.rounding = egui::Rounding::same(4.0);

    // Button styling
    visuals.widgets.inactive.bg_fill = if dark_mode {
        Color32::from_rgb(55, 65, 81)
    } else {
        Color32::from_rgb(229, 231, 235)
    };

    ctx.set_visuals(visuals);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_colors_light_mode() {
        let colors = ThemeColors::for_mode(false);
        assert_eq!(colors.bg, LIGHT_BG);
        assert_eq!(colors.text, LIGHT_TEXT);
        assert_eq!(colors.added_bg, ADDED_BG_LIGHT);
        assert_eq!(colors.deleted_bg, DELETED_BG_LIGHT);
        assert_eq!(colors.modified_bg, MODIFIED_BG_LIGHT);
    }

    #[test]
    fn test_theme_colors_dark_mode() {
        let colors = ThemeColors::for_mode(true);
        assert_eq!(colors.bg, DARK_BG);
        assert_eq!(colors.text, DARK_TEXT);
        assert_eq!(colors.added_bg, ADDED_BG_DARK);
        assert_eq!(colors.deleted_bg, DELETED_BG_DARK);
        assert_eq!(colors.modified_bg, MODIFIED_BG_DARK);
    }

    #[test]
    fn test_action_blue_values() {
        // PRD: #3B82F6 = rgb(59, 130, 246)
        assert_eq!(ACTION_BLUE, Color32::from_rgb(59, 130, 246));
    }
}
