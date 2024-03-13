#![no_std]
#![no_main]

use cortex_m_rt::entry;

// ------ debug
use defmt_rtt as _;
use embassy_stm32 as _; // Just to link it in the executable (it provides the vector table)
use panic_probe as _;

// ----- access to stm32 periphs
use embassy_stm32::rcc::*;
use embassy_stm32::Config;

pub use tp_led_matrix::matrix::Matrix;

pub use tp_led_matrix::{Color, Image};

#[entry]
fn main() -> !{
    defmt::info!("defmt correctly initialized");

    // Setup the clocks at 80MHz using HSI (by default since HSE/MSI
    // are not configured): HSI(16MHz)×10/2=80MHz. The flash wait
    // states will be configured accordingly.
    let mut config = Config::default();
    config.rcc.mux = ClockSrc::PLL1_R;
    config.rcc.hsi = true;
    config.rcc.pll = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2), // 16 * 10 / 2 = 80MHz
    });
    let p = embassy_stm32::init(config);

    // new matrix
    let mut matrix = Matrix::new(
        p.PA2, 
        p.PA3, 
        p.PA4, 
        p.PA5,
        p.PA6,
        p.PA7,
        p.PA15,
        p.PB0,
        p.PB1,
        p.PB2,
        p.PC3,
        p.PC4, 
        p.PC5, 
    );

    let image = Image::gradient(Color { r: 0, g: 0, b: 255 });

    matrix.display_image(&image);
    loop{}
}
