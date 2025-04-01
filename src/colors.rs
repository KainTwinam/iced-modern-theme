//! Modern color system definitions.
//!
//! This module provides all the standard colors from Modern's design system,
//! organized into namespaces for both light and dark modes.

use iced::Color;

/// Complete Modern system color palette
pub mod colors {
    use iced::Color;
    
    /// mobile system colors - standard variants
    pub mod system {
        use iced::Color;
        
        // Primary colors
        /// Red - used for destructive actions, errors, or high-priority alerts
        pub const RED: Color = Color::from_rgb(1.0, 0.23, 0.19);        // #FF3B30
        pub const RED_DARK: Color = Color::from_rgb(1.0, 0.27, 0.23);   // #FF453A
        
        /// Orange - used for warnings or medium-priority alerts
        pub const ORANGE: Color = Color::from_rgb(1.0, 0.58, 0.0);      // #FF9500
        pub const ORANGE_DARK: Color = Color::from_rgb(1.0, 0.62, 0.04); // #FF9F0A
        
        /// Yellow - used for cautions or minor alerts
        pub const YELLOW: Color = Color::from_rgb(1.0, 0.8, 0.0);       // #FFCC00
        pub const YELLOW_DARK: Color = Color::from_rgb(1.0, 0.84, 0.0); // #FFD60A
        
        /// Green - used for positive actions, success states, or confirmation
        pub const GREEN: Color = Color::from_rgb(0.20, 0.78, 0.35);     // #34C759
        pub const GREEN_DARK: Color = Color::from_rgb(0.19, 0.82, 0.35); // #30D158
        
        /// Mint - used for fresh, clean states
        pub const MINT: Color = Color::from_rgb(0.0, 0.78, 0.74);       // #00C7BE
        pub const MINT_DARK: Color = Color::from_rgb(0.39, 0.9, 0.89);  // #63E6E2
        
        /// Teal - blend of blue and green, professional but approachable
        pub const TEAL: Color = Color::from_rgb(0.35, 0.78, 0.98);      // #59ADC4
        pub const TEAL_DARK: Color = Color::from_rgb(0.35, 0.78, 0.98); // #59ADC4
        
        /// Cyan - bright blue-green, used for interactive elements
        pub const CYAN: Color = Color::from_rgb(0.31, 0.85, 0.98);      // #50E3FD
        pub const CYAN_DARK: Color = Color::from_rgb(0.39, 0.88, 1.0);  // #64D2FF
        
        /// Blue - primary brand color, used for interactive elements and selections
        pub const BLUE: Color = Color::from_rgb(0.0, 0.48, 1.0);        // #007AFF
        pub const BLUE_DARK: Color = Color::from_rgb(0.04, 0.52, 1.0);  // #0A84FF
        
        /// Indigo - rich violet-blue, used for depth and dimension
        pub const INDIGO: Color = Color::from_rgb(0.35, 0.34, 0.84);    // #5856D6
        pub const INDIGO_DARK: Color = Color::from_rgb(0.37, 0.36, 0.9); // #5E5CE6
        
        /// Purple - creative, imaginative color for special features
        pub const PURPLE: Color = Color::from_rgb(0.69, 0.32, 0.87);    // #AF52DE
        pub const PURPLE_DARK: Color = Color::from_rgb(0.75, 0.35, 0.95); // #BF5AF2
        
        /// Pink - playful, approachable color for light accents
        pub const PINK: Color = Color::from_rgb(1.0, 0.17, 0.34);       // #FF2D55
        pub const PINK_DARK: Color = Color::from_rgb(1.0, 0.22, 0.37);  // #FF375F
        
        /// Brown - warm, natural color for earthy or traditional UI elements
        pub const BROWN: Color = Color::from_rgb(0.64, 0.52, 0.31);     // #A2845E
        pub const BROWN_DARK: Color = Color::from_rgb(0.67, 0.56, 0.39); // #AC8E68
    }
    
    /// Gray scale colors for UI elements
    pub mod gray {
        use iced::Color;
        
        // Light mode grays
        /// Gray 1 - Lightest gray, subtle background differentiation
        pub const GRAY1_LIGHT: Color = Color::from_rgb(0.56, 0.56, 0.58); // #8E8E93
        /// Gray 2 - Light gray for secondary UI elements
        pub const GRAY2_LIGHT: Color = Color::from_rgb(0.68, 0.68, 0.7);  // #AEAEB2
        /// Gray 3 - Medium light gray for borders and separators
        pub const GRAY3_LIGHT: Color = Color::from_rgb(0.78, 0.78, 0.8);  // #C7C7CC
        /// Gray 4 - Very light gray for subtle backgrounds
        pub const GRAY4_LIGHT: Color = Color::from_rgb(0.82, 0.82, 0.84); // #D1D1D6
        /// Gray 5 - Almost white gray for card backgrounds
        pub const GRAY5_LIGHT: Color = Color::from_rgb(0.90, 0.90, 0.92); // #E5E5EA
        /// Gray 6 - Lightest gray, almost white, for main backgrounds
        pub const GRAY6_LIGHT: Color = Color::from_rgb(0.95, 0.95, 0.97); // #F2F2F7
        
        // Dark mode grays
        /// Gray 1 - Darkest gray for text and icons
        pub const GRAY1_DARK: Color = Color::from_rgb(0.56, 0.56, 0.58);  // #8E8E93
        /// Gray 2 - Dark gray for secondary UI elements
        pub const GRAY2_DARK: Color = Color::from_rgb(0.39, 0.39, 0.4);   // #636366
        /// Gray 3 - Medium dark gray for borders and separators
        pub const GRAY3_DARK: Color = Color::from_rgb(0.28, 0.28, 0.29);  // #48484A
        /// Gray 4 - Very dark gray for subtle backgrounds
        pub const GRAY4_DARK: Color = Color::from_rgb(0.22, 0.22, 0.23);  // #38383A
        /// Gray 5 - Almost black gray for card backgrounds
        pub const GRAY5_DARK: Color = Color::from_rgb(0.17, 0.17, 0.18);  // #2C2C2E
        /// Gray 6 - Darkest gray/black for main backgrounds
        pub const GRAY6_DARK: Color = Color::from_rgb(0.11, 0.11, 0.12);  // #1C1C1E
    }
    
    /// Background fill colors for UI elements
    pub mod fill {
        use iced::Color;
        
        // Light mode
        /// Primary background color (white)
        pub const BACKGROUND_LIGHT: Color = Color::WHITE;
        /// Secondary background (light gray)
        pub const SECONDARY_LIGHT: Color = Color::from_rgb(0.95, 0.95, 0.97); // #F2F2F7
        /// Tertiary background (slightly darker gray)
        pub const TERTIARY_LIGHT: Color = Color::from_rgb(0.90, 0.90, 0.92);  // #E5E5EA
        
        // Dark mode
        /// Primary background color (black/very dark gray)
        pub const BACKGROUND_DARK: Color = Color::from_rgb(0.11, 0.11, 0.12); // #1C1C1E
        /// Secondary background (dark gray)
        pub const SECONDARY_DARK: Color = Color::from_rgb(0.17, 0.17, 0.18);  // #2C2C2E
        /// Tertiary background (slightly lighter dark gray)
        pub const TERTIARY_DARK: Color = Color::from_rgb(0.22, 0.22, 0.23);   // #38383A
    }
    
    /// Text colors for various UI states
    pub mod text {
        use iced::Color;
        
        // Light mode
        /// Primary text color (black)
        pub const PRIMARY_LIGHT: Color = Color::BLACK;
        /// Secondary text color (dark gray)
        pub const SECONDARY_LIGHT: Color = Color::from_rgb(0.43, 0.43, 0.45); // #6D6D72
        /// Tertiary text color (medium gray)
        pub const TERTIARY_LIGHT: Color = Color::from_rgb(0.56, 0.56, 0.58);  // #8E8E93
        /// Quaternary text color (light gray)
        pub const QUATERNARY_LIGHT: Color = Color::from_rgb(0.68, 0.68, 0.7); // #AEAEB2
        
        // Dark mode
        /// Primary text color (white)
        pub const PRIMARY_DARK: Color = Color::WHITE;
        /// Secondary text color (light gray)
        pub const SECONDARY_DARK: Color = Color::from_rgb(0.78, 0.78, 0.8);   // #C7C7CC
        /// Tertiary text color (medium gray)
        pub const TERTIARY_DARK: Color = Color::from_rgb(0.56, 0.56, 0.58);   // #8E8E93
        /// Quaternary text color (dark gray)
        pub const QUATERNARY_DARK: Color = Color::from_rgb(0.44, 0.44, 0.46); // #6F6F74
    }
    
    /// UI element states
    pub mod state {
        use iced::Color;
        
        // Light mode
        /// Placeholder text color (for text inputs)
        pub const PLACEHOLDER_LIGHT: Color = Color::from_rgb(0.56, 0.56, 0.58); // #8E8E93
        /// Separator color (for dividers)
        pub const SEPARATOR_LIGHT: Color = Color::from_rgb(0.78, 0.78, 0.8);    // #C7C7CC
        /// Opaque separator color
        pub const OPAQUE_SEPARATOR_LIGHT: Color = Color::from_rgb(0.82, 0.82, 0.84); // #D1D1D6
        
        // Dark mode
        /// Placeholder text color (for text inputs)
        pub const PLACEHOLDER_DARK: Color = Color::from_rgb(0.56, 0.56, 0.58);  // #8E8E93
        /// Separator color (for dividers)
        pub const SEPARATOR_DARK: Color = Color::from_rgb(0.33, 0.33, 0.35);    // #545458
        /// Opaque separator color
        pub const OPAQUE_SEPARATOR_DARK: Color = Color::from_rgb(0.33, 0.33, 0.35); // #545458
    }
    
    /// macOS specific dynamic system colors
    pub mod macos {
        use iced::Color;
        
        /// Accent color - user's chosen system accent color
        pub const ACCENT: Color = Color::from_rgb(0.0, 0.48, 1.0);        // Default blue
        
        /// Control accent - color for controls in their active state
        pub const CONTROL_ACCENT: Color = Color::from_rgb(0.0, 0.48, 1.0); // Default blue
        
        /// Control background - background color for large interface elements
        pub const CONTROL_BACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.97); // #F2F2F7
        
        /// Control text - text color for available controls
        pub const CONTROL_TEXT: Color = Color::from_rgb(0.0, 0.0, 0.0);    // Black
        
        /// Find highlight - color for search result highlights
        pub const FIND_HIGHLIGHT: Color = Color::from_rgb(1.0, 0.8, 0.0);  // Yellow
        
        /// Label color - primary content text color
        pub const LABEL: Color = Color::from_rgb(0.0, 0.0, 0.0);           // Black
        
        /// Link - color for links to other content
        pub const LINK: Color = Color::from_rgb(0.0, 0.48, 1.0);           // Blue
        
        /// Selected content background - highlight color for selected content
        pub const SELECTED_CONTENT_BACKGROUND: Color = Color::from_rgb(0.0, 0.48, 1.0); // Blue
        
        /// Selected text background - background of selected text
        pub const SELECTED_TEXT_BACKGROUND: Color = Color::from_rgb(0.71, 0.84, 1.0); // Light blue
        
        /// Window background - background color for windows
        pub const WINDOW_BACKGROUND: Color = Color::from_rgb(0.95, 0.95, 0.97); // Light gray
    }
    
    /// Accessibility colors - higher contrast versions for accessibility
    pub mod accessibility {
        use iced::Color;
        
        // Light mode accessibility colors
        /// Accessible Red (Light)
        pub const RED_LIGHT: Color = Color::from_rgb(0.84, 0.0, 0.0);        // #D70000
        /// Accessible Orange (Light)
        pub const ORANGE_LIGHT: Color = Color::from_rgb(0.83, 0.33, 0.0);    // #D35400
        /// Accessible Yellow (Light)
        pub const YELLOW_LIGHT: Color = Color::from_rgb(0.77, 0.63, 0.0);    // #C4A000
        /// Accessible Green (Light)
        pub const GREEN_LIGHT: Color = Color::from_rgb(0.15, 0.54, 0.34);    // #268A57
        /// Accessible Blue (Light)
        pub const BLUE_LIGHT: Color = Color::from_rgb(0.0, 0.42, 0.87);      // #006DDE
        
        // Dark mode accessibility colors
        /// Accessible Red (Dark)
        pub const RED_DARK: Color = Color::from_rgb(1.0, 0.41, 0.38);        // #FF696185
        /// Accessible Orange (Dark)
        pub const ORANGE_DARK: Color = Color::from_rgb(1.0, 0.7, 0.4);       // #FFB366
        /// Accessible Yellow (Dark)
        pub const YELLOW_DARK: Color = Color::from_rgb(1.0, 0.83, 0.24);     // #FFD33D
        /// Accessible Green (Dark)
        pub const GREEN_DARK: Color = Color::from_rgb(0.23, 0.87, 0.57);     // #3BDF93
        /// Accessible Blue (Dark)
        pub const BLUE_DARK: Color = Color::from_rgb(0.39, 0.66, 1.0);       // #64A8FF
    }
}

// Common color constants for backward compatibility
pub const MODERN_BLUE_LIGHT: Color = colors::system::BLUE;     
pub const MODERN_BLUE_DARK: Color = colors::system::BLUE_DARK;
pub const MODERN_GREEN_LIGHT: Color = colors::system::GREEN;
pub const MODERN_GREEN_DARK: Color = colors::system::GREEN_DARK;
pub const MODERN_ORANGE_LIGHT: Color = colors::system::ORANGE;
pub const MODERN_ORANGE_DARK: Color = colors::system::ORANGE_DARK;
pub const MODERN_RED_LIGHT: Color = colors::system::RED;
pub const MODERN_RED_DARK: Color = colors::system::RED_DARK;

/// Complete set of theme colors used across the Modern-styled UI
pub struct ThemeColors {
    // Primary colors
    pub blue: Color,
    pub green: Color,
    pub orange: Color,
    pub red: Color,
    
    // UI text colors
    pub text: Color,
    pub secondary_text: Color,
    pub tertiary_text: Color,
    
    // Background colors
    pub background: Color,
    pub secondary_background: Color,
    pub tertiary_background: Color,
    
    // Element colors
    pub card_bg: Color,
    pub system_bg: Color,
    pub inactive_border: Color,
    
    // Status colors
    pub placeholder: Color,
    
    // Input elements
    pub input_bg: Color,
    pub input_border: Color,
    
    // Additional colors
    pub separator: Color,
    pub selection: Color, 
    pub link: Color,
    pub pink: Color,
    pub purple: Color,
    pub indigo: Color,
    pub teal: Color,
    pub mint: Color,
    pub yellow: Color,
    pub brown: Color,
}

/// Helper function to determine if we're in dark mode
pub fn is_dark_mode(theme: &iced::Theme) -> bool {
    match theme {
        iced::Theme::Dark => true,
        iced::Theme::Custom(custom) => custom.to_string().contains("Dark"),
        _ => false
    }
}

/// Get the appropriate colors based on theme mode
pub fn get_theme_colors(theme: &iced::Theme) -> ThemeColors {
    let is_dark = is_dark_mode(theme);
    
    ThemeColors {
        // Primary colors
        blue: if is_dark { colors::system::BLUE_DARK } else { colors::system::BLUE },
        green: if is_dark { colors::system::GREEN_DARK } else { colors::system::GREEN },
        orange: if is_dark { colors::system::ORANGE_DARK } else { colors::system::ORANGE },
        red: if is_dark { colors::system::RED_DARK } else { colors::system::RED },
        
        // UI colors
        text: if is_dark { colors::text::PRIMARY_DARK } else { colors::text::PRIMARY_LIGHT },
        secondary_text: if is_dark { colors::text::SECONDARY_DARK } else { colors::text::SECONDARY_LIGHT },
        tertiary_text: if is_dark { colors::text::TERTIARY_DARK } else { colors::text::TERTIARY_LIGHT },
        
        // Background colors
        background: if is_dark { colors::fill::BACKGROUND_DARK } else { colors::fill::BACKGROUND_LIGHT },
        secondary_background: if is_dark { colors::fill::SECONDARY_DARK } else { colors::fill::SECONDARY_LIGHT },
        tertiary_background: if is_dark { colors::fill::TERTIARY_DARK } else { colors::fill::TERTIARY_LIGHT },
        
        // Element colors
        card_bg: if is_dark { colors::fill::SECONDARY_DARK } else { colors::fill::BACKGROUND_LIGHT },
        system_bg: if is_dark { colors::gray::GRAY4_DARK } else { colors::gray::GRAY5_LIGHT },
        inactive_border: if is_dark { colors::gray::GRAY2_DARK } else { colors::gray::GRAY3_LIGHT },
        
        // Status colors
        placeholder: if is_dark { colors::state::PLACEHOLDER_DARK } else { colors::state::PLACEHOLDER_LIGHT },
        
        // Input elements
        input_bg: if is_dark { colors::fill::SECONDARY_DARK } else { colors::fill::SECONDARY_LIGHT },
        input_border: if is_dark { colors::gray::GRAY3_DARK } else { colors::gray::GRAY4_LIGHT },
        
        // Additional colors
        separator: if is_dark { colors::state::SEPARATOR_DARK } else { colors::state::SEPARATOR_LIGHT },
        selection: if is_dark { colors::system::BLUE_DARK.scale_alpha(0.3) } else { colors::system::BLUE.scale_alpha(0.3) },
        link: if is_dark { colors::system::BLUE_DARK } else { colors::system::BLUE },
        pink: if is_dark { colors::system::PINK_DARK } else { colors::system::PINK },
        purple: if is_dark { colors::system::PURPLE_DARK } else { colors::system::PURPLE },
        indigo: if is_dark { colors::system::INDIGO_DARK } else { colors::system::INDIGO },
        teal: if is_dark { colors::system::TEAL_DARK } else { colors::system::TEAL },
        mint: if is_dark { colors::system::MINT_DARK } else { colors::system::MINT },
        yellow: if is_dark { colors::system::YELLOW_DARK } else { colors::system::YELLOW },
        brown: if is_dark { colors::system::BROWN_DARK } else { colors::system::BROWN },
    }
}