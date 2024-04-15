use embassy_stm32::gpio::*;
use embassy_stm32::peripherals::*;

use crate::image::{Color, Image};

// -- For the delay function
use embassy_time::{Duration, Ticker, Timer};

pub struct Matrix<'a> {
    sb: Output<'a, PC5>,
    lat: Output<'a, PC4>,
    rst: Output<'a, PC3>,
    sck: Output<'a, PB1>,
    sda: Output<'a, PA4>,
    rows: [Output<'a, AnyPin>; 8],
}

impl Matrix<'_> {
    /// Create a new matrix from the control registers and the individual
    /// unconfigured pins. SB and LAT will be set high by default, while
    /// other pins will be set low. After 100ms, RST will be set high, and
    /// the bank 0 will be initialized by calling `init_bank0()` on the
    /// newly constructed structure.
    /// The pins will be set to very high speed mode.
    #[allow(clippy::too_many_arguments)] // Necessary to avoid a clippy warning
    pub async fn new(
        pa2: PA2, // C2
        pa3: PA3, // C7
        pa4: PA4, // SDA
        pa5: PA5,
        pa6: PA6,
        pa7: PA7,
        pa15: PA15, // <Alternate<PushPull, 0>>,
        pb0: PB0,
        pb1: PB1,
        pb2: PB2,
        pc3: PC3,
        pc4: PC4, // LAT
        pc5: PC5, // SB
    ) -> Self {
        let mut m = Matrix {
            sb: Output::new(pc5, Level::High, Speed::VeryHigh),
            lat: Output::new(pc4, Level::High, Speed::VeryHigh),
            rst: Output::new(pc3, Level::Low, Speed::VeryHigh),
            sck: Output::new(pb1, Level::Low, Speed::VeryHigh),
            sda: Output::new(pa4, Level::Low, Speed::VeryHigh),
            rows: [
                Output::new(pb2, Level::Low, Speed::VeryHigh).degrade(),
                Output::new(pa15, Level::Low, Speed::VeryHigh).degrade(),
                Output::new(pa2, Level::Low, Speed::VeryHigh).degrade(),
                Output::new(pa7, Level::Low, Speed::VeryHigh).degrade(),
                Output::new(pa6, Level::Low, Speed::VeryHigh).degrade(),
                Output::new(pa5, Level::Low, Speed::VeryHigh).degrade(),
                Output::new(pb0, Level::Low, Speed::VeryHigh).degrade(),
                Output::new(pa3, Level::Low, Speed::VeryHigh).degrade(),
            ],
        };

        Timer::after(Duration::from_millis(100)).await;

        // Set RST high
        m.rst.set_high();

        // Initialize bank0
        m.init_bank0();

        m
    }

    /// Make a brief high pulse of the SCK pin
    fn pulse_sck(&mut self) {
        // Set SCK high
        self.sck.set_high();
        // Set SCK low
        self.sck.set_low();
    }

    /// Make a brief low pulse of the LAT pin
    fn pulse_lat(&mut self) {
        // Set LAT low
        self.lat.set_low();
        // Set LAT high
        self.lat.set_high();
    }

    /// Send a byte on SDA starting with the MSB and pulse SCK high after each bit
    fn send_byte(&mut self, pixel: u8) {
        for i in (0..8).rev() {
            // Set SDA to the i-th bit of pixel
            if (pixel >> i) & 1 == 1 {
                self.sda.set_high();
            } else {
                self.sda.set_low();
            }
            // Pulse SCK
            self.pulse_sck();
        }
    }

    /// Send a full row of bytes in BGR order and pulse LAT low. Gamma correction
    /// must be applied to every pixel before sending them. The previous row must
    /// be deactivated and the new one activated.
    pub fn send_row(&mut self, row: usize, pixels: &[Color]) {
        // Deactivate the previous row
        if row > 0 {
            self.rows[row - 1].set_low();
        } else {
            self.rows[7].set_low();
        }

        // Send the new row
        for pixel in pixels {
            // Apply gamma correction to the pixel
            let color = pixel.gamma_correct();

            // Send the pixel in BGR order
            self.send_byte(color.b);
            self.send_byte(color.g);
            self.send_byte(color.r);
        }

        // Pulse LAT low
        self.pulse_lat();

        // Activate the new row
        self.rows[row].set_high();
    }

    /// Initialize bank0 by temporarily setting SB to low and sending 144 one bits,
    /// pulsing SCK high after each bit and pulsing LAT low at the end. SB is then
    /// restored to high.
    fn init_bank0(&mut self) {
        // Set SB low
        self.sb.set_low();

        // Send 144 one bits
        for _ in 0..144 {
            self.sda.set_high();
            self.pulse_sck();
        }

        // Pulse LAT low
        self.pulse_lat();

        // Set SB high
        self.sb.set_high();
    }

    /// Display a full image, row by row, as fast as possible.
    pub async fn display_image(&mut self, image: &Image, ticker: &mut Ticker) {
        // Do not forget that image.row(n) gives access to the content of row n,
        // and that self.send_row() uses the same format.
        for row in 0..8 {
            ticker.next().await;
            self.send_row(row, image.row(row));
        }
    }
}
