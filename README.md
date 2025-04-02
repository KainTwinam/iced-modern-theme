<div align="center">

# Iced Modern Inspired Theme
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A comprehensive Modern-inspired theme for [Iced](https://github.com/iced-rs/iced), the cross-platform GUI library for Rust.

<a href="https://github.com/KainTwinam/iced-modern-theme">
  <img src="/assets/dark-theme.gif" width="460px">
</a>
<a href="https://github.com/KainTwinam/iced-modern-theme">
  <img src="/assets/light-theme.gif" width="460px">
</a>

</div>

## Features

- Complete set of Modern like system colors for both light and dark modes
- Styled components matching Modern's design language:
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
use iced_Modern_theme::Modern;

struct MyApp {
    theme: Theme,
    // ...
}

impl MyApp {
    // ...
    
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
    
    // ...
}

fn main() -> iced::Result {
    iced::application("MyApp Title", MyApp::update, MyApp::view)
        .theme(MyApp::theme)
        .run_with(MyApp::new)
}
```

## Examples

### Button Styles

```rust
use iced::widget::Button;
use iced_Modern_theme::Modern;

// Primary button (blue filled)
Button::new("Primary")
    .style(Modern::primary_button());

// Secondary button (outlined)
Button::new("Secondary")
    .style(Modern::secondary_button());

// Success button (green)
Button::new("Success")
    .style(Modern::success_button());

// Warning button (orange/yellow)
Button::new("Warning")
    .style(Modern::warning_button());

// Danger button (red)
Button::new("Danger")
    .style(Modern::danger_button());

// Tinted button (semi-transparent colored background)
Button::new("Tinted")
    .style(Modern::blue_tinted_button());
```

### Container Styles

```rust
use iced::widget::Container;
use iced_Modern_theme::Modern;

// Card container (with shadow and rounded corners)
Container::new(content)
    .style(Modern::card_container());

// Sheet container (modal-like appearance)
Container::new(content)
    .style(Modern::sheet_container());

// Floating container (elevated with shadow)
Container::new(content)
    .style(Modern::floating_container());
```

### Text Input Styles

```rust
use iced::widget::TextInput;
use iced_Modern_theme::Modern;

// Standard text input
TextInput::new("Placeholder", &value)
    .style(Modern::text_input());

// Search input with rounded corners
TextInput::new("Search...", &value)
    .style(Modern::search_input());

// Inline text input with bottom border only
TextInput::new("Inline input", &value)
    .style(Modern::inline_text_input());
```

### Text Styles

```rust
use iced::widget::Text;
use iced_Modern_theme::Modern;

// Primary text (main content)
Text::new("Primary text")
    .style(Modern::primary_text());

// Secondary text (supporting information)
Text::new("Secondary text")
    .style(Modern::secondary_text());

// Colored text
Text::new("Red error message")
    .style(Modern::red_text());

Text::new("Blue information")
    .style(Modern::blue_text());
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
let light_theme = Modern::light_theme();

// Dark mode
let dark_theme = Modern::dark_theme();
```

## Available Colors

The theme includes all standard Modern system colors:

- **Primary Colors**: Red, Orange, Yellow, Green, Mint, Teal, Cyan, Blue, Indigo, Purple, Pink, Brown
- **Gray Scale**: Multiple levels of gray for UI elements
- **Semantic Colors**: For text, backgrounds, borders, etc.

Each color has appropriate light and dark mode variants that are automatically selected based on the current theme.

## Custom Components

You can create custom components using the Modern styling functions:

```rust
// Custom button style
let my_custom_button = button(row![icon, text("Custom Button")])
    .style(Modern::primary_button())
    .padding(8);

// Custom container
let my_custom_panel = container(content)
    .style(Modern::card_container())
    .padding(15)
    .width(Length::Fill);
```

## Credits

This theme is inspired by modern design principles similar to those used in various contemporary operating systems and applications. 
It is not affiliated with or endorsed by Apple Inc. or any other company.

## License

This project is licensed under the MIT License - see the LICENSE file for details.