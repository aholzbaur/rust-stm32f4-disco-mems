#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::{entry};
use stm32f4xx_hal as hal;
use hal::{prelude::*,
          stm32,
          spi::{Spi, Mode, Phase, Polarity},
          delay::Delay};

#[entry]
fn start() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    let core_periphs = cortex_m::peripheral::Peripherals::take().unwrap();

    device_periphs.RCC.apb2enr.write(|w| w.syscfgen().enabled());

    let rcc_periph = device_periphs.RCC.constrain();

    let clocks = rcc_periph.cfgr
        .use_hse(8.mhz()) // discovery board has 8 MHz crystal for HSE
        .hclk(180.mhz())
        .sysclk(180.mhz())
        .pclk1(45.mhz())
        .pclk2(90.mhz())
        .freeze();

    let gpioc = device_periphs.GPIOC.split();
    let gpiof = device_periphs.GPIOF.split();

    let mut cs = gpioc.pc1.into_push_pull_output();
    cs.set_high().unwrap();

    let sck = gpiof.pf7.into_alternate_af5();
    let miso = gpiof.pf8.into_alternate_af5();
    let mosi = gpiof.pf9.into_alternate_af5();

    let mut spi = Spi::spi5(
        device_periphs.SPI5,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleHigh,
            phase: Phase::CaptureOnFirstTransition,
        },
        2.mhz().into(),
        clocks,
    );

    // CS low -> enable gyro communication
    cs.set_low().unwrap();

    let mut delay = Delay::new(core_periphs.SYST, clocks);
    delay.delay_ms(100_u32);

    let b = 0x8F_u8; // read WHO_AM_I register
    spi.send(b).unwrap();
    while spi.is_txe() == false {
    }
    let dummy = 0x00_u8;
    spi.send(dummy).unwrap();
    while spi.is_txe() == false {
    }
    let result= spi.read();
    let reg_val = match result {
        Ok(value) => value,
        Err(error) => panic!("Error: {:?}", error)
    };

    // WHO_AM_I should be 215 according to manual
    if reg_val != 0xD7_u8 {
        panic!("Wrong register value! {:?}", reg_val);
    }

    loop {
        // The main thread can now go to sleep.
        // WFI (wait for interrupt) puts the core in sleep until an interrupt occurs.
        cortex_m::asm::wfi();
    }
}