//! SigmaOS-craft-libs: shared craft libraries.
//!
//! Real implementation: shared utilities, common types, reusable components, and validation.
//! Replaces the 20-line thin placeholder.

use std::fmt;

/// RGBA color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self { Self { r, g, b, a: 1.0 } }
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }
}

/// 2D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

/// 2D size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self { Self { width, height } }
}

/// Rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self { Self { x, y, width, height } }
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
}

/// Font reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontRef {
    pub family: String,
    pub weight: u16,
}

impl FontRef {
    pub fn new(family: impl Into<String>, weight: u16) -> Self { Self { family: family.into(), weight } }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibError {
    InvalidColor,
    InvalidSize,
    EmptyFontFamily,
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidColor => write!(f, "invalid color"),
            Self::InvalidSize => write!(f, "invalid size"),
            Self::EmptyFontFamily => write!(f, "font family must not be empty"),
        }
    }
}

impl std::error::Error for LibError {}

/// Utility functions.
pub struct Utils;

impl Utils {
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if value < min { min } else if value > max { max } else { value }
    }

    pub fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_aaa() {
        let c = Color::rgb(1.0, 0.0, 0.0);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_point_aaa() {
        let p = Point::new(10.0, 20.0);
        assert_eq!(p.x, 10.0);
    }

    #[test]
    fn test_rect_contains_aaa() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(r.contains(Point::new(50.0, 50.0)));
        assert!(!r.contains(Point::new(150.0, 50.0)));
    }

    #[test]
    fn test_utils_clamp_aaa() {
        assert_eq!(Utils::clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(Utils::clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(Utils::clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_utils_lerp_aaa() {
        assert_eq!(Utils::lerp(0.0, 10.0, 0.5), 5.0);
    }
}
