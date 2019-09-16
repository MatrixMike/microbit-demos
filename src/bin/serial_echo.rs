#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use nb::block;
use nrf51::Peripherals;
use nrf51_hal::prelude::*;
use cortex_m::asm::*;
use nrf51_hal::serial::{Serial, BAUD115200};

#[entry]
fn main() -> ! {
    if let Some(p) = Peripherals::take() {
        let gpio = p.GPIO.split();
        let tx_pin = gpio.pin24.into_push_pull_output().downgrade();
        let rx_pin = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, mut rx) = Serial::uart0(p.UART0, tx_pin, rx_pin, BAUD115200).split();
        loop {
            let val = block!(rx.read()).unwrap();
            let _ = block!(tx.write(val));
            nop();   // comment 
        }
    }
    panic!();
}
