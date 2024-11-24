#![no_std]
#![no_main]

extern crate alloc;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::SpiMode;
use esp_hal::timer::timg::TimerGroup;
use log::info;

/// Possible command register values on the display chip.
#[derive(Clone, Copy)]
pub enum Register {
    Noop = 0x00,
    Digit0 = 0x01,
    Digit1 = 0x02,
    Digit2 = 0x03,
    Digit3 = 0x04,
    Digit4 = 0x05,
    Digit5 = 0x06,
    Digit6 = 0x07,
    Digit7 = 0x08,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Power = 0x0C,
    DisplayTest = 0x0F,
}

impl From<Register> for u8 {
    fn from(command: Register) -> u8 {
        command as u8
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);
    esp_println::logger::init_logger_from_env();

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    info!("Embassy initialized!");

    let _ = spawner;

    let mosi = peripherals.GPIO4;
    let cs = peripherals.GPIO5;
    let sclk = peripherals.GPIO6;
    let miso = peripherals.GPIO7;

    let mut spi_async = Spi::new_with_config(
        peripherals.SPI2,
        Config {
            frequency: 1_000_000.Hz(),
            mode: SpiMode::Mode0,
            ..Config::default()
        },
    )
        .with_sck(sclk)
        .with_mosi(mosi)
        .with_miso(miso)
        .with_cs(cs)
        .into_async();

    // let mut noop: [u8; 2] = [Register::Noop.into(); 2];

    // off
    info!("off");
    let mut buf: [u8; 2] = [Register::Power.into(), 0x00];
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    info!("on");
    let mut buf: [u8; 2] = [Register::Power.into(), 0x01];
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    // spi_async.write_bytes(&mut noop).unwrap();

    // decode mode
    info!("set decode mode");
    let mut buf: [u8; 2] = [Register::DecodeMode.into(), 0x00];
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    // spi_async.write_bytes(&mut noop).unwrap();

    // scan limit
    info!("scan limit");
    let mut buf: [u8; 2] = [Register::ScanLimit.into(), 0x07];
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    // spi_async.write_bytes(&mut noop).unwrap();

    // intensity
    info!("intensity");
    let mut buf: [u8; 2] = [Register::Intensity.into(), 0x00];
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    spi_async.write_bytes(&mut buf).unwrap();
    // spi_async.write_bytes(&mut noop).unwrap();

    let mut i: u8 = 0;

    // let bpm = 120;
    loop {
        let bit = i % 8;
        info!("{:b} | {:b}", i, bit);

        let mut buf: [u8; 2] = [Register::Digit0.into(), 1 << bit];
        spi_async.write_bytes(&mut buf).unwrap();
        let mut buf: [u8; 2] = [Register::Digit0.into(), 1 << bit];
        spi_async.write_bytes(&mut buf).unwrap();
        let mut buf: [u8; 2] = [Register::Digit0.into(), 1 << bit];
        spi_async.write_bytes(&mut buf).unwrap();
        let mut buf: [u8; 2] = [Register::Digit0.into(), 1 << bit];
        spi_async.write_bytes(&mut buf).unwrap();

        // Timer::after(Duration::from_millis(1000)).await;

        let mut buf: [u8; 2] = [Register::Digit1.into(), 1 << bit];
        spi_async.write_bytes(&mut buf).unwrap();
        let mut buf: [u8; 2] = [Register::Digit1.into(), 1 << bit];
        spi_async.write_bytes(&mut buf).unwrap();
        let mut buf: [u8; 2] = [Register::Digit1.into(), 1 << bit];
        spi_async.write_bytes(&mut buf).unwrap();
        let mut buf: [u8; 2] = [Register::Digit1.into(), 1 << bit];
        spi_async.write_bytes(&mut buf).unwrap();

        // let mut buf: [u8; 2] = [Register::DecodeMode.into(), 0x07, Register::Noop.into(), Register::Noop.into(), Register::Noop.into()];
        // spi_async.write_bytes(&mut buf).unwrap();
        // let mut buf: [u8; 2] = [Register::DecodeMode.into(), 0x07];
        // spi_async.write_bytes(&mut buf).unwrap();
        // spi_async.write_bytes(&mut noop).unwrap();
        // set display intensity lower
        Timer::after(Duration::from_millis(125)).await;

        if i == 0xff {
            i = 0
        } else {
            i += 1;
        }
    }
}
