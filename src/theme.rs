//! Modern theme implementation for Iced GUI components.
//!
//! This module provides the main `Modern` struct and implementations
//! for styling each Iced component with Modern-inspired designs.

use iced::{Border, Color, Shadow, Theme, Background, Vector};
use iced::overlay::menu;

/// Modern design-inspired text input style implementation
fn text_input_style(theme: &Theme, status: TextInputStatus) -> text_input::Style {
    let colors = get_theme_colors(theme);
    
    let base_style = text_input::Style {
        background: Background::Color(colors.input_bg),
        border: Border {
            radius: SMALL_CORNER_RADIUS.into(),
            width: 1.0,
            color: colors.input_border,
        },
        icon: colors.text,
        placeholder: colors.placeholder,
        value: colors.text,
        selection: colors.blue.scale_alpha(0.3),
    };
    
    match status {
        TextInputStatus::Active => base_style,
        TextInputStatus::Hovered => text_input::Style {
            border: Border {
                color: colors.placeholder,
                ..base_style.border
            },
            ..base_style
        },
        TextInputStatus::Focused => text_input::Style {
            border: Border {
                color: colors.blue,
                width: 2.0,
                ..base_style.border
            },
            ..base_style
        },
        TextInputStatus::Disabled => text_input::Style {
            background: Background::Color(colors.input_bg.scale_alpha(0.7)),
            border: Border {
                color: colors.input_border.scale_alpha(0.5),
                ..base_style.border
            },
            value: colors.text.scale_alpha(0.5),
            ..base_style
        },
    }
}

/// Modern design-inspired pick list style implementation
fn pick_list_style(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let colors = get_theme_colors(theme);
    
    // Base style
    let base_style = pick_list::Style {
        text_color: colors.text,
        placeholder_color: colors.placeholder,
        background: Background::Color(colors.input_bg),
        border: Border {
            radius: SMALL_CORNER_RADIUS.into(),
            width: 1.0,
            color: colors.input_border,
        },
        handle_color: colors.placeholder,
    };
    
    match status {
        pick_list::Status::Active => base_style,
        pick_list::Status::Hovered => pick_list::Style {
            border: Border {
                color: colors.placeholder,
                ..base_style.border
            },
            ..base_style
        },
        pick_list::Status::Opened => pick_list::Style {
            border: Border {
                color: colors.blue,
                width: 1.5,
                ..base_style.border
            },
            handle_color: colors.blue,
            ..base_style
        },
    }
}

/// Modern design-inspired combo box style implementation
fn combo_box_style(theme: &Theme, status: TextInputStatus) -> text_input::Style {
    // For consistency, we use the same style as text input
    text_input_style(theme, status)
}

/// Create a complete Modern-styled theme
fn create_modern_theme(dark_mode: bool) -> Theme {
    let name = if dark_mode { "Modern Dark" } else { "Modern Light" };
    
    // Define the base colors
    let (background, text) = if dark_mode {
        (Color::from_rgb(0.11, 0.11, 0.12), Color::WHITE) // #1C1C1E (dark bg)
    } else {
        (Color::from_rgb(0.95, 0.95, 0.97), Color::BLACK) // #F2F2F7 (light bg)
    };
    
    let primary = if dark_mode { MODERN_BLUE_DARK } else { MODERN_BLUE_LIGHT };
    let success = if dark_mode { MODERN_GREEN_DARK } else { MODERN_GREEN_LIGHT };
    let danger = if dark_mode { MODERN_RED_DARK } else { MODERN_RED_LIGHT };
    
    // Create the Modern theme
    Theme::custom(
        String::from(name),
        iced::theme::Palette {
            background,
            text,
            primary,
            success,
            danger,
        }
    )
}

/// Modern design-inspired radio button style implementation
fn radio_style(theme: &Theme, status: radio::Status) -> radio::Style {
    let colors = get_theme_colors(theme);
    
    // Base style
    let style = radio::Style {
        background: Background::Color(Color::TRANSPARENT),
        dot_color: colors.blue,
        border_width: 2.0,
        border_color: match status {
            radio::Status::Active { is_selected } if is_selected => colors.blue,
            radio::Status::Hovered { is_selected } if is_selected => colors.blue,
            _ => colors.inactive_border,
        },
        text_color: Some(colors.text),
    };
    
    // Adjust for hover state
    match status {
        radio::Status::Hovered { is_selected: true } => style,
        radio::Status::Hovered { is_selected: false } => radio::Style {
            border_color: colors.blue.scale_alpha(0.5),
            ..style
        },
        _ => style,
    }
}

/// Modern design-inspired checkbox style implementation
fn checkbox_style(theme: &Theme, status: checkbox::Status) -> checkbox::Style {
    let colors = get_theme_colors(theme);
    
    match status {
        checkbox::Status::Active { is_checked } => {
            if is_checked {
                checkbox::Style {
                    background: Background::Color(colors.blue),
                    icon_color: Color::WHITE,
                    border: Border {
                        radius: TINY_CORNER_RADIUS.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: Some(colors.text),
                }
            } else {
                checkbox::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    icon_color: Color::TRANSPARENT,
                    border: Border {
                        radius: TINY_CORNER_RADIUS.into(),
                        width: 2.0,
                        color: colors.inactive_border,
                    },
                    text_color: Some(colors.text),
                }
            }
        },
        checkbox::Status::Hovered { is_checked } => {
            if is_checked {
                checkbox::Style {
                    background: Background::Color(colors.blue.scale_alpha(0.9)),
                    icon_color: Color::WHITE,
                    border: Border {
                        radius: TINY_CORNER_RADIUS.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: Some(colors.text),
                }
            } else {
                checkbox::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    icon_color: Color::TRANSPARENT,
                    border: Border {
                        radius: TINY_CORNER_RADIUS.into(),
                        width: 2.0,
                        color: colors.blue.scale_alpha(0.5),
                    },
                    text_color: Some(colors.text),
                }
            }
        },
        checkbox::Status::Disabled { is_checked } => {
            if is_checked {
                checkbox::Style {
                    background: Background::Color(colors.blue.scale_alpha(0.5)),
                    icon_color: Color::WHITE.scale_alpha(0.5),
                    border: Border {
                        radius: TINY_CORNER_RADIUS.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: Some(colors.text.scale_alpha(0.5)),
                }
            } else {
                checkbox::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    icon_color: Color::TRANSPARENT,
                    border: Border {
                        radius: TINY_CORNER_RADIUS.into(),
                        width: 2.0,
                        color: colors.inactive_border.scale_alpha(0.5),
                    },
                    text_color: Some(colors.text.scale_alpha(0.5)),
                }
            }
        },
    }
}

/// Modern design-inspired container style
fn container_style(theme: &Theme, class: &style::Container) -> container::Style {
    let colors = get_theme_colors(theme);
    
    match class {
        style::Container::Transparent => container::Style {
            text_color: Some(colors.text),
            background: None,
            border: Border::default(),
            shadow: Shadow::default(),
        },
        
        style::Container::Card => {
            container::Style {
                text_color: Some(colors.text),
                background: Some(Background::Color(colors.card_bg)),
                border: Border {
                    radius: 10.0.into(), // Modern rounded card corners
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.1, ..Color::BLACK },
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            }
        },
        
        style::Container::Sheet => {
            let sheet_bg = if is_dark_mode(theme) {
                Color::from_rgb(0.22, 0.22, 0.23) // #383839 (dark mode sheet)
            } else {
                Color::from_rgb(0.95, 0.95, 0.97) // #F2F2F7 (light mode sheet)
            };
            
            container::Style {
                text_color: Some(colors.text),
                background: Some(Background::Color(sheet_bg)),
                border: Border {
                    radius: 12.0.into(), // Modern rounded sheet corners
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.2, ..Color::BLACK },
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
            }
        },
        
        style::Container::Group => {
            let group_bg = if is_dark_mode(theme) {
                Color::from_rgb(0.17, 0.17, 0.18) // #2C2C2E (dark mode group)
            } else {
                Color::from_rgb(0.95, 0.95, 0.97) // #F2F2F7 (light mode group)
            };
            
            container::Style {
                text_color: Some(colors.text),
                background: Some(Background::Color(group_bg)),
                border: Border {
                    radius: 10.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(), // No shadow for groups
            }
        },
        
        style::Container::Sidebar => {
            let sidebar_bg = if is_dark_mode(theme) {
                Color::from_rgb(0.15, 0.15, 0.16) // #262628 (dark mode sidebar)
            } else {
                Color::from_rgb(0.92, 0.92, 0.93) // #EAEAEE (light mode sidebar)
            };
            
            container::Style {
                text_color: Some(colors.text),
                background: Some(Background::Color(sidebar_bg)),
                border: Border::default(),
                shadow: Shadow {
                    color: Color { a: 0.05, ..Color::BLACK },
                    offset: Vector::new(1.0, 0.0),
                    blur_radius: 3.0,
                },
            }
        },
    }
}

fn button_hover_style(base_style: button::Style, is_dark: bool) -> button::Style {
    let adjust_color = |color: Color| -> Color {
        if is_dark {
            // Lighten in dark mode
            Color {
                r: (color.r + 0.05).min(1.0),
                g: (color.g + 0.05).min(1.0),
                b: (color.b + 0.05).min(1.0),
                a: color.a,
            }
        } else {
            // Darken in light mode
            Color {
                r: (color.r - 0.05).max(0.0),
                g: (color.g - 0.05).max(0.0),
                b: (color.b - 0.05).max(0.0),
                a: color.a,
            }
        }
    };
    
    if let Some(Background::Color(color)) = base_style.background {
        button::Style {
            background: Some(Background::Color(adjust_color(color))),
            ..base_style
        }
    } else {
        base_style
    }
}

fn button_pressed_style(base_style: button::Style, is_dark: bool) -> button::Style {
    let adjust_color = |color: Color| -> Color {
        if is_dark {
            // Lighten more in dark mode
            Color {
                r: (color.r + 0.1).min(1.0),
                g: (color.g + 0.1).min(1.0),
                b: (color.b + 0.1).min(1.0),
                a: color.a,
            }
        } else {
            // Darken more in light mode
            Color {
                r: (color.r - 0.1).max(0.0),
                g: (color.g - 0.1).max(0.0),
                b: (color.b - 0.1).max(0.0),
                a: color.a,
            }
        }
    };
    
    let mut pressed_style = base_style;
    pressed_style.shadow = Shadow::default(); // Remove shadow when pressed
    
    if let Some(Background::Color(color)) = base_style.background {
        pressed_style.background = Some(Background::Color(adjust_color(color)));
    }
    
    pressed_style
}

fn button_disabled_style(base_style: button::Style) -> button::Style {
    button::Style {
        background: base_style.background.map(|bg| match bg {
            Background::Color(color) => Background::Color(color.scale_alpha(0.5)),
            _ => bg,
        }),
        text_color: base_style.text_color.scale_alpha(0.5),
        border: Border {
            color: base_style.border.color.scale_alpha(0.5),
            ..base_style.border
        },
        shadow: Shadow::default(), // No shadow for disabled buttons
    }
}

use iced::widget::{button, text, text_input, container, radio, checkbox, pick_list, combo_box};
use iced::widget::button::Status as ButtonStatus;
use iced::widget::text_input::Status as TextInputStatus;

use crate::colors::*;
use crate::styles::*;

/// Modern theme utilities for styling iced widgets
pub struct Modern;

impl Modern {
    /// Get an Modern-style theme for buttons
    pub fn button<'a>(style: style::Button) -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| button_style(theme, &style, status)
    }

    /// Get an Modern-style theme for primary buttons (blue)
    pub fn primary_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::button(style::Button::Primary)
    }

    /// Get an Modern-style theme for secondary buttons (outlined)
    pub fn secondary_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::button(style::Button::Secondary)
    }

    /// Get an Modern-style theme for success buttons (green)
    pub fn success_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::button(style::Button::Success)
    }

    /// Get an Modern-style theme for warning buttons (orange)
    pub fn warning_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::button(style::Button::Warning)
    }

    /// Get an Modern-style theme for danger buttons (red)
    pub fn danger_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::button(style::Button::Danger)
    }

    /// Get an Modern-style theme for link buttons (text-only)
    pub fn link_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::button(style::Button::Link)
    }

    /// Get an Modern-style theme for system buttons (light gray)
    pub fn system_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::button(style::Button::System)
    }

    /// Get an Modern-style theme for plain buttons (text only without link color)
    pub fn plain_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::button(style::Button::Plain)
    }

    /// Get an Modern-style theme for text inputs
    pub fn text_input<'a>() -> impl Fn(&Theme, TextInputStatus) -> text_input::Style + 'a {
        text_input_style
    }

    /// Get an Modern-style theme for containers
    pub fn container<'a>(style: style::Container) -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| container_style(theme, &style)
    }

    /// Get an Modern-style theme for card containers
    pub fn card_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        Self::container(style::Container::Card)
    }

    /// Get an Modern-style theme for sheet containers
    pub fn sheet_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        Self::container(style::Container::Sheet)
    }

    /// Get an Modern-style theme for group containers
    pub fn group_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        Self::container(style::Container::Group)
    }

    /// Get an Modern-style theme for sidebar containers
    pub fn sidebar_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        Self::container(style::Container::Sidebar)
    }

    /// Get an Modern-style theme for radio buttons
    pub fn radio<'a>() -> impl Fn(&Theme, radio::Status) -> radio::Style + 'a {
        radio_style
    }

    /// Get an Modern-style theme for checkboxes
    pub fn checkbox<'a>() -> impl Fn(&Theme, checkbox::Status) -> checkbox::Style + 'a {
        checkbox_style
    }

    /// Get an Modern-style theme for pick lists
    pub fn pick_list<'a>() -> impl Fn(&Theme, pick_list::Status) -> pick_list::Style + 'a {
        pick_list_style
    }

/*     /// Get an Modern-style theme for combo boxes
    pub fn combo_box<'a>() -> impl Fn(&Theme, TextInputStatus) -> text_input::Style + 'a {
        combo_box_style
    } */

    /// Create a complete Modern-styled theme
    pub fn theme(dark_mode: bool) -> Theme {
        create_modern_theme(dark_mode)
    }

    /// Create a light Modern-styled theme
    pub fn light_theme() -> Theme {
        Self::theme(false)
    }

    /// Create a dark Modern-styled theme
    pub fn dark_theme() -> Theme {
        Self::theme(true)
    }

    // Additional button styles using more Modern colors
    
    /// Get a teal button style (cyan-blue)
    pub fn teal_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            let is_dark = is_dark_mode(theme);
            
            let modern_base = |color: Color, text_color: Color| button::Style {
                background: Some(Background::Color(color)),
                text_color,
                border: Border {
                    radius: CORNER_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.1, ..Color::BLACK },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
            };
            
            let base_style = modern_base(colors.teal, Color::WHITE);
            
            match status {
                ButtonStatus::Active => base_style,
                ButtonStatus::Hovered => button_hover_style(base_style, is_dark),
                ButtonStatus::Pressed => button_pressed_style(base_style, is_dark),
                ButtonStatus::Disabled => button_disabled_style(base_style),
            }
        }
    }
    
    /// Get an indigo button style (blue-purple)
    pub fn indigo_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            let is_dark = is_dark_mode(theme);
            
            let modern_base = |color: Color, text_color: Color| button::Style {
                background: Some(Background::Color(color)),
                text_color,
                border: Border {
                    radius: CORNER_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.1, ..Color::BLACK },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
            };
            
            let base_style = modern_base(colors.indigo, Color::WHITE);
            
            match status {
                ButtonStatus::Active => base_style,
                ButtonStatus::Hovered => button_hover_style(base_style, is_dark),
                ButtonStatus::Pressed => button_pressed_style(base_style, is_dark),
                ButtonStatus::Disabled => button_disabled_style(base_style),
            }
        }
    }
    
    /// Get a purple button style
    pub fn purple_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            let is_dark = is_dark_mode(theme);
            
            let modern_base = |color: Color, text_color: Color| button::Style {
                background: Some(Background::Color(color)),
                text_color,
                border: Border {
                    radius: CORNER_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.1, ..Color::BLACK },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
            };
            
            let base_style = modern_base(colors.purple, Color::WHITE);
            
            match status {
                ButtonStatus::Active => base_style,
                ButtonStatus::Hovered => button_hover_style(base_style, is_dark),
                ButtonStatus::Pressed => button_pressed_style(base_style, is_dark),
                ButtonStatus::Disabled => button_disabled_style(base_style),
            }
        }
    }
    
    /// Get a pink button style
    pub fn pink_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            let is_dark = is_dark_mode(theme);
            
            let modern_base = |color: Color, text_color: Color| button::Style {
                background: Some(Background::Color(color)),
                text_color,
                border: Border {
                    radius: CORNER_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.1, ..Color::BLACK },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
            };
            
            let base_style = modern_base(colors.pink, Color::WHITE);
            
            match status {
                ButtonStatus::Active => base_style,
                ButtonStatus::Hovered => button_hover_style(base_style, is_dark),
                ButtonStatus::Pressed => button_pressed_style(base_style, is_dark),
                ButtonStatus::Disabled => button_disabled_style(base_style),
            }
        }
    }

     /// Get an Modern-style gray button (neutral, subdued appearance)
     pub fn gray_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            let is_dark = is_dark_mode(theme);
            
            // Gray color varies by theme
            let gray_color = if is_dark {
                colors::gray::GRAY3_DARK
            } else {
                colors::gray::GRAY4_LIGHT
            };
            
            let modern_base = |color: Color, text_color: Color| button::Style {
                background: Some(Background::Color(color)),
                text_color,
                border: Border {
                    radius: CORNER_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.1, ..Color::BLACK }, 
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
            };
            
            let base_style = modern_base(gray_color, colors.text);
            
            match status {
                ButtonStatus::Active => base_style,
                ButtonStatus::Hovered => button_hover_style(base_style, is_dark),
                ButtonStatus::Pressed => button_pressed_style(base_style, is_dark),
                ButtonStatus::Disabled => button_disabled_style(base_style),
            }
        }
    }
    
    /// Get an Modern-style tinted button (semi-transparent colored background)
    pub fn tinted_button<'a>(color_variant: TintedButtonColor) -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            let is_dark = is_dark_mode(theme);
            
            // Get the base color based on the variant
            let (base_color, text_color) = match color_variant {
                TintedButtonColor::Blue => (colors.blue, Color::WHITE),
                TintedButtonColor::Green => (colors.green, Color::WHITE),
                TintedButtonColor::Red => (colors.red, Color::WHITE),
                TintedButtonColor::Orange => (colors.orange, Color::WHITE),
                TintedButtonColor::Purple => (colors.purple, Color::WHITE),
                TintedButtonColor::Teal => (colors.teal, Color::WHITE),
                TintedButtonColor::Pink => (colors.pink, Color::WHITE),
                TintedButtonColor::Indigo => (colors.indigo, Color::WHITE),
            };
            
            // Make color semi-transparent for tinted look
            let tinted_color = Color {
                r: base_color.r,
                g: base_color.g,
                b: base_color.b,
                a: 0.2, // Low opacity for tinted appearance
            };
            
            // For tinted buttons, we usually want a stronger text color
            let modern_base = |color: Color, text_color: Color| button::Style {
                background: Some(Background::Color(color)),
                text_color,
                border: Border {
                    radius: CORNER_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.05, ..Color::BLACK }, // Lighter shadow for tinted
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
            };
            
            let base_style = modern_base(tinted_color, base_color); // Use the base color for text
            
            match status {
                ButtonStatus::Active => base_style,
                ButtonStatus::Hovered => button_hover_style(base_style, is_dark),
                ButtonStatus::Pressed => button_pressed_style(base_style, is_dark),
                ButtonStatus::Disabled => button_disabled_style(base_style),
            }
        }
    }
    
    /// Get an Modern-style blue tinted button
    pub fn blue_tinted_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::tinted_button(TintedButtonColor::Blue)
    }
    
    /// Get an Modern-style green tinted button
    pub fn green_tinted_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::tinted_button(TintedButtonColor::Green)
    }
    
    /// Get an Modern-style red tinted button
    pub fn red_tinted_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::tinted_button(TintedButtonColor::Red)
    }
    
    /// Get an Modern-style orange tinted button
    pub fn orange_tinted_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::tinted_button(TintedButtonColor::Orange)
    }
    
    /// Get an Modern-style purple tinted button
    pub fn purple_tinted_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::tinted_button(TintedButtonColor::Purple)
    }
    
    /// Get an Modern-style teal tinted button
    pub fn teal_tinted_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::tinted_button(TintedButtonColor::Teal)
    }
    
    /// Get an Modern-style indigo tinted button
    pub fn indigo_tinted_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::tinted_button(TintedButtonColor::Indigo)
    }
    
    /// Get an Modern-style pink tinted button
    pub fn pink_tinted_button<'a>() -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        Self::tinted_button(TintedButtonColor::Pink)
    }
    
    /// Size variants for buttons (small, medium, large)
    pub fn sized_button<'a>(
        style_fn: impl Fn(&Theme, ButtonStatus) -> button::Style + 'a,
        size: ButtonSize
    ) -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| {
            let mut base_style = style_fn(theme, status);
            
            // Modify the border radius based on size
            base_style.border = Border {
                radius: match size {
                    ButtonSize::Small => (CORNER_RADIUS * 0.8).into(),
                    ButtonSize::Medium => CORNER_RADIUS.into(),
                    ButtonSize::Large => (CORNER_RADIUS * 1.2).into(),
                },
                ..base_style.border
            };
            
            base_style
        }
    }


    /// Create a "selected" version of any button style
    /// 
    /// This function takes any button style and creates a modified version where
    /// the default appearance matches what would normally be the pressed state.
    /// This is perfect for navigation items to show which item is currently selected.
    pub fn selected_button_style<'a>(
        base_style_fn: impl Fn(&Theme, ButtonStatus) -> button::Style + 'a
    ) -> impl Fn(&Theme, ButtonStatus) -> button::Style + 'a {
        move |theme, status| {
            let is_dark = is_dark_mode(theme);
            
            match status {
                ButtonStatus::Active => {
                    // For the active state, use what would normally be the pressed state
                    let base_style = base_style_fn(theme, ButtonStatus::Active);
                    button_pressed_style(base_style, is_dark)
                },
                // For other states, use the original style function
                _ => base_style_fn(theme, status),
            }
        }
    }
    
    // Container variants
    
    /// Get a container with separator line style
    pub fn separated_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            let colors = get_theme_colors(theme);
            
            container::Style {
                text_color: Some(colors.text),
                background: Some(Background::Color(colors.background)),
                border: Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
            }
        }
    }
    
    /// Get a branded container with accent color border
    pub fn accent_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            let colors = get_theme_colors(theme);
            
            container::Style {
                text_color: Some(colors.text),
                background: Some(Background::Color(colors.background)),
                border: Border {
                    radius: 8.0.into(),
                    width: 2.0,
                    color: colors.blue,
                },
                shadow: Shadow {
                    color: Color { a: 0.1, ..Color::BLACK },
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 4.0,
                },
            }
        }
    }

    /// Get a toolbar container style
    pub fn toolbar_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            let colors = get_theme_colors(theme);
            
            container::Style {
                text_color: Some(colors.text),
                background: Some(Background::Color(colors.system_bg)),
                border: Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.05, ..Color::BLACK },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
            }
        }
    }
    
    /// Get a floating panel container style
    pub fn floating_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            let colors = get_theme_colors(theme);
            
            container::Style {
                text_color: Some(colors.text),
                background: Some(Background::Color(colors.card_bg)),
                border: Border {
                    radius: 10.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow {
                    color: Color { a: 0.25, ..Color::BLACK },
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
            }
        }
    }

    /// Get a danger tooltip container style with error styling
    pub fn danger_tooltip_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            let colors = get_theme_colors(theme);
    
            // Determine if dark mode
            let dark_mode = is_dark_mode(theme);
    
            container::Style {
                text_color: Some(if dark_mode {
                    Color::from_rgb(1.0, 0.6, 0.6) // lighter red for better contrast
                } else {
                    Color::from_rgb(0.7, 0.0, 0.0) // slightly darker red for better readability
                }),
                background: Some(Background::Color(
                    if dark_mode {
                        Color { r: 0.4, g: 0.1, b: 0.1, a: 1.0 }  // More saturated dark red
                    } else {
                        Color { r: 1.0, g: 0.92, b: 0.92, a: 1.0 }  // softer, clearer red
                    }
                )),
                border: Border {
                    radius: 6.0.into(),
                    width: 1.0,
                    color: if dark_mode {
                        Color::from_rgb(0.8, 0.3, 0.3)
                    } else {
                        Color::from_rgb(0.9, 0.6, 0.6)
                    },
                },
                shadow: Shadow {
                    color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.15 },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 3.0,
                },
            }
        }
    }

    /// Get a warning tooltip container style with warning styling
    pub fn warning_tooltip_container<'a>() -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            let colors = get_theme_colors(theme);
            
            container::Style {
                text_color: Some(colors.orange),  // Orange text for warnings
                background: Some(Background::Color(
                    if is_dark_mode(theme) {
                        // Darker theme - subtle orange-brown background
                        Color { r: 0.3, g: 0.15, b: 0.0, a: 0.7 }
                    } else {
                        // Light theme - very subtle light orange background
                        Color { r: 1.0, g: 0.96, b: 0.9, a: 1.0 }
                    }
                )),
                border: Border {
                    radius: 6.0.into(),  // Slightly rounded corners
                    width: 1.0,
                    color: colors.orange,  // Orange border to match the warning theme
                },
                shadow: Shadow {
                    color: Color { a: 0.1, ..Color::BLACK },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
            }
        }
    }

    /// Dynamically choose between danger, warning and standard tooltip container styles
    pub fn conditional_tooltip_container<'a>(
        validation_state: ValidationState
    ) -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            match validation_state {
                ValidationState::Error => {
                    // Call the danger function and immediately apply it
                    (Self::danger_tooltip_container())(theme)
                },
                ValidationState::Warning => {
                    // Call the warning function and immediately apply it
                    (Self::warning_tooltip_container())(theme)
                },
                ValidationState::Valid => {
                    // Call a standard container style for consistent padding/sizing
                    // You could use card_container or create a specific info_tooltip_container
                    (Self::card_container())(theme)
                }
            }
        }
    }

    ///  Simple conditional container if you don't want / need a warning state
    pub fn validated_tooltip_container<'a>(
        has_error: bool
    ) -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            if has_error {
                // Call the danger function and immediately apply it
                (Self::danger_tooltip_container())(theme)
            } else {
                // Call a standard container style for consistent padding/sizing
                (Self::card_container())(theme)
            }
        }
    }
    
    
    // Text input variants
    
    /// Get a search input style with rounded corners
    pub fn search_input<'a>() -> impl Fn(&Theme, TextInputStatus) -> text_input::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            
            let base_style = text_input::Style {
                background: Background::Color(colors.system_bg),
                border: Border {
                    radius: CORNER_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                icon: colors.tertiary_text,
                placeholder: colors.placeholder,
                value: colors.text,
                selection: colors.selection,
            };
            
            match status {
                TextInputStatus::Active => base_style,
                TextInputStatus::Hovered => base_style,
                TextInputStatus::Focused => text_input::Style {
                    background: Background::Color(colors.tertiary_background),
                    ..base_style
                },
                TextInputStatus::Disabled => text_input::Style {
                    background: Background::Color(colors.system_bg.scale_alpha(0.7)),
                    value: colors.text.scale_alpha(0.5),
                    ..base_style
                },
            }
        }
    }
    
    /// Get an inline text input style with bottom border only
    pub fn inline_text_input<'a>() -> impl Fn(&Theme, TextInputStatus) -> text_input::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            
            let base_style = text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: Border {
                    radius: 0.0.into(),
                    width: 1.0,
                    color: colors.separator,
                },
                icon: colors.text,
                placeholder: colors.placeholder,
                value: colors.text,
                selection: colors.selection,
            };
            
            match status {
                TextInputStatus::Active => base_style,
                TextInputStatus::Hovered => text_input::Style {
                    border: Border {
                        color: colors.tertiary_text,
                        ..base_style.border
                    },
                    ..base_style
                },
                TextInputStatus::Focused => text_input::Style {
                    border: Border {
                        color: colors.blue,
                        width: 2.0,
                        ..base_style.border
                    },
                    ..base_style
                },
                TextInputStatus::Disabled => text_input::Style {
                    border: Border {
                        color: colors.separator.scale_alpha(0.5),
                        ..base_style.border
                    },
                    value: colors.text.scale_alpha(0.5),
                    ..base_style
                },
            }
        }
    }

    /// Get an modern danger theme for text inputs with validation errors
    pub fn danger_text_input<'a>() -> impl Fn(&Theme, TextInputStatus) -> text_input::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            
            // Start with base text input style
            let base_style = text_input_style(theme, status);
            
            // Override with error styling
            text_input::Style {
                border: Border {
                    color: colors.red,  // Use red border for error indication
                    width: 1.0,
                    ..base_style.border
                },
                // You could add a light red background for increased visibility
                background: Background::Color(
                    if is_dark_mode(theme) {
                        // Darker theme - subtle dark red
                        Color { r: 0.3, g: 0.0, b: 0.0, a: 0.2 }
                    } else {
                        // Light theme - very subtle light red
                        Color { r: 1.0, g: 0.9, b: 0.9, a: 1.0 }
                    }
                ),
                ..base_style
            }
        }
    }

    /// Dynamically choose between danger, warning and inline text input styles
    pub fn conditional_text_input<'a>(
        validation_state: ValidationState
    ) -> impl Fn(&Theme, TextInputStatus) -> text_input::Style + 'a {
        move |theme, status| {
            match validation_state {
                ValidationState::Error => {
                    // Call the danger function and immediately apply it
                    (Self::danger_text_input())(theme, status)
                },
                ValidationState::Warning => {
                    // Call the warning function and immediately apply it
                    (Self::warning_text_input())(theme, status)
                },
                ValidationState::Valid => {
                    // Call the inline function and immediately apply it
                    (Self::inline_text_input())(theme, status)
                }
            }
        }
    }

    // Simple conditional text_input if you don't need/want a warning state
    pub fn validated_text_input<'a>(
        has_error: bool
    ) -> impl Fn(&Theme, TextInputStatus) -> text_input::Style + 'a {
        move |theme, status| {
            if has_error {
                (Self::danger_text_input())(theme, status)
            } else {
                (Self::inline_text_input())(theme, status)
            }
        }
    }
    
    // Get an modern warning theme for text inputs with validation errors
    pub fn warning_text_input<'a>() -> impl Fn(&Theme, TextInputStatus) -> text_input::Style + 'a {
        move |theme, status| {
            let colors = get_theme_colors(theme);
            let base_style = text_input_style(theme, status);
            
            text_input::Style {
                border: Border {
                    color: colors.orange,  // Orange border for warnings
                    width: 1.0,
                    ..base_style.border
                },
                ..base_style
            }
        }
    }

    /// Get an Modern-style primary text style (main content text)
    pub fn primary_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        |theme| {
            let colors = get_theme_colors(theme);
            
            text::Style {
                color: Some(colors.text),
            }
        }
    }
    
    /// Get an Modern-style secondary text style (supporting information)
    pub fn secondary_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        |theme| {
            let colors = get_theme_colors(theme);
            
            text::Style {
                color: Some(colors.secondary_text),
            }
        }
    }
    
    /// Get an Modern-style tertiary text style (less important information)
    pub fn tertiary_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        |theme| {
            let colors = get_theme_colors(theme);
            
            text::Style {
                color: Some(colors.tertiary_text),
            }
        }
    }
    
    /// Get an Modern-style link text style
    pub fn link_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        |theme| {
            let colors = get_theme_colors(theme);
            
            text::Style {
                color: Some(colors.link),
            }
        }
    }
    
    /// Get text styled with a specific color for both light and dark modes
    pub fn colored_text<'a>(
        light_color: Color,
        dark_color: Color
    ) -> impl Fn(&Theme) -> text::Style + 'a {
        move |theme| {
            let is_dark = is_dark_mode(theme);
            let color = if is_dark { dark_color } else { light_color };
            
            text::Style {
                color: Some(color),
            }
        }
    }
    
    /// Get text in Modern's red color
    pub fn red_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::RED, colors::system::RED_DARK)
    }
    
    /// Get text in Modern's blue color
    pub fn blue_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::BLUE, colors::system::BLUE_DARK)
    }
    
    /// Get text in Modern's green color
    pub fn green_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::GREEN, colors::system::GREEN_DARK)
    }
    
    /// Get text in Modern's orange color
    pub fn orange_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::ORANGE, colors::system::ORANGE_DARK)
    }
    
    /// Get text in Modern's yellow color
    pub fn yellow_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::YELLOW, colors::system::YELLOW_DARK)
    }
    
    /// Get text in Modern's purple color
    pub fn purple_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::PURPLE, colors::system::PURPLE_DARK)
    }
    
    /// Get text in Modern's pink color
    pub fn pink_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::PINK, colors::system::PINK_DARK)
    }
    
    /// Get text in Modern's teal color
    pub fn teal_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::TEAL, colors::system::TEAL_DARK)
    }
    
    /// Get text in Modern's indigo color
    pub fn indigo_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::INDIGO, colors::system::INDIGO_DARK)
    }
    
    /// Get text in Modern's mint color
    pub fn mint_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::MINT, colors::system::MINT_DARK)
    }
    
    /// Get text in Modern's brown color
    pub fn brown_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::colored_text(colors::system::BROWN, colors::system::BROWN_DARK)
    }
    
    /// Get a success / positive message text style
    pub fn success_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::green_text()
    }
    
    /// Get a warning message text style
    pub fn warning_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::orange_text()
    }
    
    /// Get an error / destructive message text style
    pub fn error_text<'a>() -> impl Fn(&Theme) -> text::Style + 'a {
        Self::red_text()
    }

    /// Get an modern theme for combo boxes
    pub fn combo_box<'a>() -> impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a {
        // Use the same style as text_input, since combo_box uses TextInput under the hood
        text_input_style
    }

    /// Get a modern theme for combo box menus
    pub fn combo_box_menu<'a>() -> impl Fn(&Theme) -> menu::Style + 'a {
        |theme| {
            let colors = get_theme_colors(theme);
            
            menu::Style {
                text_color: colors.text,
                background: Background::Color(colors.card_bg),
                border: Border {
                    radius: TINY_CORNER_RADIUS.into(),
                    width: 1.0,
                    color: colors.input_border,
                },
                selected_text_color: Color::WHITE,
                selected_background: Background::Color(colors.blue),
            }
        }
    }

    /// Conditional button style helper specifically for button styles
    pub fn conditional_button_style<'a>(
        condition: bool,
        true_style: impl Fn(&Theme, button::Status) -> button::Style + 'a,
        false_style: impl Fn(&Theme, button::Status) -> button::Style + 'a,
    ) -> impl Fn(&Theme, button::Status) -> button::Style + 'a {
        move |theme, status| {
            if condition {
                true_style(theme, status)
            } else {
                false_style(theme, status)
            }
        }
    }

    // Similar helpers for other widget types
    /// Conditional text input style helper
    pub fn conditional_text_input_style<'a>(
        condition: bool,
        true_style: impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a,
        false_style: impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a,
    ) -> impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a {
        move |theme, status| {
            if condition {
                true_style(theme, status)
            } else {
                false_style(theme, status)
            }
        }
    }

    /// Conditional container style helper
    pub fn conditional_container_style<'a>(
        condition: bool,
        true_style: impl Fn(&Theme) -> container::Style + 'a,
        false_style: impl Fn(&Theme) -> container::Style + 'a,
    ) -> impl Fn(&Theme) -> container::Style + 'a {
        move |theme| {
            if condition {
                true_style(theme)
            } else {
                false_style(theme)
            }
        }
    }
}

/// Modern design-inspired button style implementation
fn button_style(theme: &Theme, class: &style::Button, status: ButtonStatus) -> button::Style {
    let colors = get_theme_colors(theme);
    let is_dark = is_dark_mode(theme);
    
    // Function to create the base Modern style with rounded corners
    let modern_base = |color: Color, text_color: Color| button::Style {
        background: Some(Background::Color(color)),
        text_color,
        border: Border {
            radius: CORNER_RADIUS.into(), // Modern's rounded corners
            width: 0.0,         // No border for filled buttons
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color { a: 0.1, ..Color::BLACK }, 
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
    };
    
    // Function to create outlined style
    let outlined = |color: Color, text_color: Color| button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color,
        border: Border {
            radius: CORNER_RADIUS.into(),
            width: 1.0,
            color,
        },
        shadow: Shadow::default(),
    };
    
    // Function to create a transparent button style (for links/text)
    let transparent = |text_color: Color| button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color,
        border: Border::default(),
        shadow: Shadow::default(),
    };
    
    // Base style based on button class
    let base_style = match class {
        style::Button::Primary => modern_base(colors.blue, Color::WHITE),
        style::Button::Secondary => outlined(colors.blue, colors.blue),
        style::Button::Success => modern_base(colors.green, Color::WHITE),
        style::Button::Warning => modern_base(colors.orange, if is_dark { Color::BLACK } else { Color::WHITE }),
        style::Button::Danger => modern_base(colors.red, Color::WHITE),
        style::Button::Link => transparent(colors.blue),
        style::Button::System => modern_base(colors.system_bg, colors.text),
        style::Button::Plain => transparent(colors.text),
    };
    
    // Adjust style based on status
    match status {
        ButtonStatus::Active => base_style,
        
        ButtonStatus::Hovered => {
            // For Modern style, make buttons slightly lighter/darker on hover
            let adjust_color = |color: Color| -> Color {
                if is_dark {
                    // Lighten in dark mode
                    Color {
                        r: (color.r + 0.05).min(1.0),
                        g: (color.g + 0.05).min(1.0),
                        b: (color.b + 0.05).min(1.0),
                        a: color.a,
                    }
                } else {
                    // Darken in light mode
                    Color {
                        r: (color.r - 0.05).max(0.0),
                        g: (color.g - 0.05).max(0.0),
                        b: (color.b - 0.05).max(0.0),
                        a: color.a,
                    }
                }
            };
            
            match class {
                style::Button::Link | style::Button::Plain => {
                    // For text/links, just adjust the text color
                    button::Style {
                        text_color: base_style.text_color.scale_alpha(0.8),
                        ..base_style
                    }
                },
                _ => {
                    // For other buttons, adjust the background
                    if let Some(Background::Color(color)) = base_style.background {
                        button::Style {
                            background: Some(Background::Color(adjust_color(color))),
                            ..base_style
                        }
                    } else {
                        base_style
                    }
                }
            }
        },
        
        ButtonStatus::Pressed => {
            // For pressed state, make buttons even more light/dark and reduce shadow
            let adjust_color = |color: Color| -> Color {
                if is_dark {
                    // Lighten more in dark mode
                    Color {
                        r: (color.r + 0.1).min(1.0),
                        g: (color.g + 0.1).min(1.0),
                        b: (color.b + 0.1).min(1.0),
                        a: color.a,
                    }
                } else {
                    // Darken more in light mode
                    Color {
                        r: (color.r - 0.1).max(0.0),
                        g: (color.g - 0.1).max(0.0),
                        b: (color.b - 0.1).max(0.0),
                        a: color.a,
                    }
                }
            };
            
            let mut pressed_style = base_style;
            
            // Remove shadow when pressed (Modern's buttons appear to press down)
            pressed_style.shadow = Shadow::default();
            
            match class {
                style::Button::Link | style::Button::Plain => {
                    // For text/links, just adjust the text color more
                    pressed_style.text_color = base_style.text_color.scale_alpha(0.6);
                    pressed_style
                },
                _ => {
                    // For other buttons, adjust the background more
                    if let Some(Background::Color(color)) = base_style.background {
                        pressed_style.background = Some(Background::Color(adjust_color(color)));
                    }
                    pressed_style
                }
            }
        },
        
        ButtonStatus::Disabled => {
            // For disabled state, reduce opacity
            button::Style {
                background: base_style.background.map(|bg| match bg {
                    Background::Color(color) => Background::Color(color.scale_alpha(0.5)),
                    _ => bg,
                }),
                text_color: base_style.text_color.scale_alpha(0.5),
                border: Border {
                    color: base_style.border.color.scale_alpha(0.5),
                    ..base_style.border
                },
                shadow: Shadow::default(), // No shadow for disabled buttons
            }
        },
    }
    
}

// Define an enum for validation states
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationState {
    Valid,
    Warning,
    Error,
}
