#[path = "gamma.rs"]
mod gamma;

// Constants
pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

//----------- Color structure ------------
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

//----------- Image structure ------------
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Image([Color; 64]);

impl Image {
    pub const fn new_solid(color: Color) -> Self {
        Image([color; 64])
    }

    pub fn row(&self, row: usize) -> &[Color] {
        &self.0[row * 8..(row + 1) * 8]
    }

    pub fn gradient(color: Color) -> Self {
        let mut image: Image = Default::default();
        for row in 0..8 {
            for col in 0..8 {
                image[(row, col)] = color / (1 + row * row + col) as f32;
            }
        }
        image
    }
}

impl Default for Image {
    fn default() -> Self {
        Image([Color::default(); 64])
    }
}

impl core::ops::Index<(usize, usize)> for Image {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Color {
        &self.0[x * 8 + y]
    }
}

impl core::ops::IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Color {
        &mut self.0[y * 8 + x]
    }
}

impl AsRef<[u8; 192]> for Image {
    fn as_ref(&self) -> &[u8; 192] {
        unsafe { &*(self as *const Self as *const [u8; 192]) }
    }
}

impl AsMut<[u8; 192]> for Image {
    fn as_mut(&mut self) -> &mut [u8; 192] {
        unsafe { &mut *(self as *mut Self as *mut [u8; 192]) }
    }
}

impl Image {
    pub fn from_buffer(buff: &[u8; 192]) -> Image {
        let mut image = Image::default();

        for row in 0..8 {
            for col in 0..8 {
                image[(row, col)] = Color {
                    r: buff[row * 24 + col * 3],
                    g: buff[row * 24 + col * 3 + 1],
                    b: buff[row * 24 + col * 3 + 2],
                };
            }
        }
        image
    }
}
