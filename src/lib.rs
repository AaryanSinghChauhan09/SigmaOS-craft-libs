#[derive(Debug, Clone, PartialEq)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }
impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self { Self { r, g, b, a: 1.0 } }
}

#[derive(Debug, Clone)]
pub struct FontRef { pub family: String, pub weight: u16 }
impl FontRef {
    pub fn new(family: impl Into<String>, weight: u16) -> Self { Self { family: family.into(), weight } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_color() {
        let c = Color::rgb(1.0, 0.0, 0.0);
        assert_eq!(c.r, 1.0);
    }
}
