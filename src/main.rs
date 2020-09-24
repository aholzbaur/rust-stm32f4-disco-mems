#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::{entry};
use stm32f4xx_hal as hal;
use hal::{prelude::*,
          stm32};

#[entry]
fn start() -> ! {
    let device_periphs = stm32::Peripherals::take().unwrap();
    
    device_periphs.RCC.apb2enr.write(|w| w.syscfgen().enabled());

    let rcc_periph = device_periphs.RCC.constrain();
    
    let _clocks = rcc_periph.cfgr
        .use_hse(8.mhz()) // discovery board has 8 MHz crystal for HSE
        .hclk(180.mhz())
        .sysclk(180.mhz())
        .pclk1(45.mhz())
        .pclk2(90.mhz())
        .freeze();

    loop {
        // The main thread can now go to sleep.
        // WFI (wait for interrupt) puts the core in sleep until an interrupt occurs.
        cortex_m::asm::wfi();
    }
}