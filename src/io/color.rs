use std::ops;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        let ret = |a: u8| -> u8{
            (a as f32 * rhs) as u8
        };
        Color::with_rgba(ret(self.r), ret(self.g), ret(self.b), ret(self.a))
    }
}

impl Color {
    pub fn red() -> Self {Color::with_rgb(255, 0, 0)}
    pub fn blue() -> Self {Color::with_rgb(0, 0, 255)}
    pub fn green() -> Self {Color::with_rgb(0, 255, 0)}
    pub fn black() -> Self {Color::with_rgb(0, 0, 0)}
    pub fn white() -> Self {Color::with_rgb(255, 255, 255)}
}

impl Color {
    pub fn with_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn with_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn as_u8(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn set_to(&mut self, color: Color){
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
        self.a = color.a;
    }
}
