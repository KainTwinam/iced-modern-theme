# Iced Apple Inspired Theme
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A comprehensive Apple-inspired theme for [Iced](https://github.com/iced-rs/iced), the cross-platform GUI library for Rust.

## Features

- Complete set of Apple like system colors for both light and dark modes
- Styled components matching Apple's design language:
  - Buttons (primary, secondary, tinted, etc.)
  - Text inputs (standard, search, inline)
  - Containers (card, sheet, sidebar, etc.)
  - Form controls (checkbox, radio, pick list)
  - Text styles with semantic variations
- Ready-to-use theme that can be applied to any Iced application
- Automatic light/dark mode support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iced = "0.13.1"
palette = "0.7.5" 
```

## Quick Start

```rust
use iced::{Application, Settings, Theme};
use iced_apple_theme::Apple;

struct MyApp {
    theme: Theme,
    // ...
}

impl Application for MyApp {
    // ...
    
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
    
    // ...
}

fn main() -> iced::Result {
    MyApp::run(Settings {
        default_theme: Some(Apple::light_theme()),
        // ...
    })
}
```

## Examples

### Button Styles

```rust
use iced::widget::Button;
use iced_apple_theme::Apple;

// Primary button (blue filled)
Button::new("Primary")
    .style(Apple::primary_button());

// Secondary button (outlined)
Button::new("Secondary")
    .style(Apple::secondary_button());

// Success button (green)
Button::new("Success")
    .style(Apple::success_button());

// Warning button (orange/yellow)
Button::new("Warning")
    .style(Apple::warning_button());

// Danger button (red)
Button::new("Danger")
    .style(Apple::danger_button());

// Tinted button (semi-transparent colored background)
Button::new("Tinted")
    .style(Apple::blue_tinted_button());
```

### Container Styles

```rust
use iced::widget::Container;
use iced_apple_theme::Apple;

// Card container (with shadow and rounded corners)
Container::new(content)
    .style(Apple::card_container());

// Sheet container (modal-like appearance)
Container::new(content)
    .style(Apple::sheet_container());

// Floating container (elevated with shadow)
Container::new(content)
    .style(Apple::floating_container());
```

### Text Input Styles

```rust
use iced::widget::TextInput;
use iced_apple_theme::Apple;

// Standard text input
TextInput::new("Placeholder", &value)
    .style(Apple::text_input());

// Search input with rounded corners
TextInput::new("Search...", &value)
    .style(Apple::search_input());

// Inline text input with bottom border only
TextInput::new("Inline input", &value)
    .style(Apple::inline_text_input());
```

### Text Styles

```rust
use iced::widget::Text;
use iced_apple_theme::Apple;

// Primary text (main content)
Text::new("Primary text")
    .style(Apple::primary_text());

// Secondary text (supporting information)
Text::new("Secondary text")
    .style(Apple::secondary_text());

// Colored text
Text::new("Red error message")
    .style(Apple::red_text());

Text::new("Blue information")
    .style(Apple::blue_text());
```

## Showcase Example

Check out the `showcase` example to see all available styles and components:

```bash
cargo run --example showcase
```

## Light and Dark Mode

The theme automatically adapts to light and dark mode. You can explicitly set the mode:

```rust
// Light mode
let light_theme = Apple::light_theme();

// Dark mode
let dark_theme = Apple::dark_theme();
```

## Available Colors

The theme includes all standard Apple system colors:

- **Primary Colors**: Red, Orange, Yellow, Green, Mint, Teal, Cyan, Blue, Indigo, Purple, Pink, Brown
- **Gray Scale**: Multiple levels of gray for UI elements
- **Semantic Colors**: For text, backgrounds, borders, etc.

Each color has appropriate light and dark mode variants that are automatically selected based on the current theme.

## Custom Components

You can create custom components using the Apple styling functions:

```rust
// Custom button style
let my_custom_button = button(row![icon, text("Custom Button")])
    .style(Apple::primary_button())
    .padding(8);

// Custom container
let my_custom_panel = container(content)
    .style(Apple::card_container())
    .padding(15)
    .width(Length::Fill);
```

## Credits

This theme is inspired by Apple's Human Interface Guidelines and design system. It aims to provide a familiar and polished look for applications built with Iced.

## License

This project is licensed under the MIT License - see the LICENSE file for details.