#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
// ------ debug
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32 as _;

// use embassy_sync::mutex::MutexGuard as _;
use embassy_stm32::usart::Uart;

// Just to link it in the executable (it provides the vector table)
use embassy_stm32::gpio::*;
use embassy_stm32::peripherals::*;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Ticker, Timer};
use panic_probe as _;

// ----- access to stm32 periphs
use embassy_stm32::rcc::*;
use embassy_stm32::Config;

pub use tp_led_matrix::matrix::Matrix;

pub use tp_led_matrix::{Color, Image};

// ---- For Mutex
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;

// import GREEN
use tp_led_matrix::image::{BLUE, GREEN, RED};

// ---- Image static
static IMAGE: Mutex<ThreadModeRawMutex, Image> = Mutex::new(Image::new_solid(GREEN));

// ----- For serial communication
use embassy_stm32::bind_interrupts;
use embassy_stm32::dma::NoDma as noDma;
use embassy_stm32::usart::{self, DataBits, Parity, StopBits};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<USART1>;
});

// Triple buff
use futures::FutureExt;
use heapless::{
    box_pool,
    pool::boxed::{Box, BoxBlock},
};
box_pool!(POOL: Image);
static mut NEXT_IMAGE: Signal<ThreadModeRawMutex, Box<POOL>> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> () {
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
    let matrix = Matrix::new(
        p.PA2, p.PA3, p.PA4, p.PA5, p.PA6, p.PA7, p.PA15, p.PB0, p.PB1, p.PB2, p.PC3, p.PC4, p.PC5,
    )
    .await;
    unsafe {
        #[allow(clippy::declare_interior_mutable_const)]
        const BLOCK: BoxBlock<Image> = BoxBlock::new();
        static mut MEMORY: [BoxBlock<Image>; 3] = [BLOCK; 3];
        #[allow(static_mut_refs)]
        for block in &mut MEMORY {
            POOL.manage(block);
        }
    }

    // change the image
    let mut image = IMAGE.lock().await;
    *image = Image::gradient(Color { r: 0, g: 255, b: 0 });
    drop(image);

    let _ = spawner.spawn(blinker(p.PB14));
    let _ = spawner.spawn(display(matrix));
    // let _ = spawner.spawn(chaging_image());

    // Create the Uart device. Use NoDma for the transmit DMA, as we will not transmit anything, we don't need to block a DMA channel which might be useful for another peripheral. Also, don't forget to configure the baudrate to 38400.
    let mut config = embassy_stm32::usart::Config::default();
    config.baudrate = 38400_u32;
    config.data_bits = DataBits::DataBits8;
    config.stop_bits = StopBits::STOP1;
    config.parity = Parity::ParityNone;

    let uart: Uart<'static, USART1, noDma, DMA1_CH5> =
        Uart::new(p.USART1, p.PB7, p.PB6, Irqs, noDma, p.DMA1_CH5, config).unwrap();

    // task for serial receiver
    let _ = spawner.spawn(serial_receiver(uart));

    let mut i: u8 = 0;
    loop {
        let image = POOL.alloc(match i {
            0 => Image::gradient(Color { r: 0, g: 255, b: 0 }),
            1 => Image::gradient(Color { r: 0, g: 0, b: 255 }),
            2 => Image::gradient(Color { r: 255, g: 0, b: 0 }),
            3 => Image::new_solid(RED),
            4 => Image::new_solid(GREEN),
            5 => Image::new_solid(BLUE),
            _ => Image::new_solid(Color { r: 0, g: 0, b: 0 }),
        });
        if i == 5 {
            i = 0;
        }
        i += 1;
        // send image
        unsafe {
            NEXT_IMAGE.signal(image.unwrap());
        }
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn blinker(p: PB14) {
    // init the port as output
    let mut led = Output::new(p, Level::Low, Speed::VeryHigh);
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn display(mut matrix: Matrix<'static>) {
    let mut ticker = Ticker::every(Duration::from_hz(640));
    let mut row_buffer: &[Color];

    let mut image_option: Option<Box<POOL>>;
    let mut image: Image = Image::gradient(GREEN);

    loop {
        unsafe {
            image_option = NEXT_IMAGE.wait().now_or_never();
        }
        match image_option {
            Some(new_image) => {
                image = *new_image;

                for row in 0..8 {
                    ticker.next().await;
                    row_buffer = image.row(row);
                    matrix.send_row(row, row_buffer);
                }
            }
            None => {
                // If no image is available, we just display the current image.;
                for row in 0..8 {
                    ticker.next().await;
                    row_buffer = image.row(row);
                    matrix.send_row(row, row_buffer);
                }
            }
        }
    }
}

#[embassy_executor::task]
async fn chaging_image() {
    loop {
        Timer::after(Duration::from_millis(1000)).await;
        *IMAGE.lock().await = Image::new_solid(RED);
        Timer::after(Duration::from_millis(1000)).await;
        *IMAGE.lock().await = Image::new_solid(GREEN);
        Timer::after(Duration::from_millis(1000)).await;
        *IMAGE.lock().await = Image::new_solid(BLUE);
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn serial_receiver(mut uart: Uart<'static, USART1, noDma, DMA1_CH5>) {
    let mut n = 0;
    let mut buffer = [0u8; 192];

    loop {
        // Receive the missing 192-N bytes starting at offset N of the buffer
        for i in n..192 {
            let _ = uart.read(&mut buffer[i..i + 1]).await;
            if buffer[i] == 0xff {
                n = i;
            }
        }

        // --- Remark -- that if we send 0xff|o|o|o|oxff|n|n|n we get n|n|n|0xff|o|o|o|oxff
        // which is normal if we did exacly what the "Receiving the image efficiently" part says
        // as we cannot get rid of the very first 0xff without doing a special case.
        if n > 0 {
            buffer.rotate_right(n - 1);
            n = 0;
        }
        defmt::info!("{:?}", buffer);

        // updating the image
        {
            let temp = Image::from_buffer(&buffer);
            let mut image = IMAGE.lock().await;
            *image = temp;
        }
    }
}
