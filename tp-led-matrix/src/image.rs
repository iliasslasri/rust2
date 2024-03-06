#![no_std]

use micromath::F32Ext;

#[path = "gamma.rs"] mod gamma;

// Constants
const RED: Color = Color { r: 255, g: 0, b: 0 };
const GREEN: Color = Color { r: 0, g: 255, b: 0 };
const BLUE: Color = Color { r: 0, g: 0, b: 255 };

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
}

impl Color {
    pub fn gamma_correct(&self) -> Self {
        Color {
            r: gamma::gamma_correct(self.r),
            g: gamma::gamma_correct(self.g),
            b: gamma::gamma_correct(self.b),
        }
    }
}

impl core::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, f: f32) -> Color {
        Color {
            r: (self.r as f32 * f).clamp(0.0, 255.0) as u8,
            g: (self.g as f32 * f).clamp(0.0, 255.0) as u8,
            b: (self.b as f32 * f).clamp(0.0, 255.0) as u8,
        }
    }
}

// core::ops::Div<f32>
impl core::ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, f: f32) -> Color {
        self * (1.0 / f)
    }
}
