//! Style definitions for Apple-inspired UI components.
//!
//! This module provides the style variants and enum definitions
//! used throughout the Apple theme.

/// Common constants for element sizing
pub const CORNER_RADIUS: f32 = 8.0;
pub const SMALL_CORNER_RADIUS: f32 = 6.0;
pub const TINY_CORNER_RADIUS: f32 = 4.0;

/// Apple-styled component variants
pub mod style {
    /// Button style variants
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Button {
        /// Primary filled button (blue)
        Primary,
        /// Secondary outlined button
        Secondary,
        /// Success/positive action button (green)
        Success,
        /// Warning action button (orange/yellow)
        Warning,
        /// Danger/destructive action button (red)
        Danger,
        /// Link-styled button (appears as a text link)
        Link,
        /// System button (light gray background)
        System,
        /// Plain text button
        Plain,
    }

    /// Container style variants
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Container {
        /// Standard transparent container
        Transparent,
        /// Card style with background
        Card,
        /// Sheet/modal style
        Sheet,
        /// Group style (for grouped tables/lists)
        Group,
        /// Sidebar style
        Sidebar,
    }

    impl Default for Button {
        fn default() -> Self {
            Self::Primary
        }
    }

    impl Default for Container {
        fn default() -> Self {
            Self::Transparent
        }
    }
}

/// Colors available for tinted buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TintedButtonColor {
    Blue,
    Green,
    Red,
    Orange,
    Purple,
    Teal,
    Pink,
    Indigo,
}

/// Size variants for buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}