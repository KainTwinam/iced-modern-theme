//! # iced-apple-theme
//!
//! A collection of Apple-inspired theme components for the Iced GUI library.
//!
//! This crate provides a comprehensive set of styles and colors that emulate
//! the look and feel of Apple's design system for macOS, iOS, and iPadOS.
//!
//! ## Features
//!
//! - Complete set of Apple system colors for both light and dark modes
//! - Styled components: buttons, text inputs, containers, etc.
//! - Variants for different button styles (primary, secondary, tinted, etc.)
//! - Ready-to-use theme that can be applied to any Iced application

pub mod colors;
pub mod styles;
pub mod theme;

pub use colors::*;
pub use styles::*;
pub use theme::*;

/// Re-export the Apple struct as the main entry point for the library
pub use theme::Apple;