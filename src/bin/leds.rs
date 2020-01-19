#![no_main]
#![no_std]
// please do not expect this to compile properly under geany without more assistance
extern crate panic_halt;
// github.com therealprof / microbit
use core::fmt::Write;
use cortex_m_rt::entry;
use microbit::led::Display;
//use microbit::hal::i2c;
use nrf51::Peripherals;
use nrf51_hal::delay::Delay;
use nrf51_hal::prelude::*;
use nrf51_hal::serial::{Serial, BAUD115200};

#[entry]
/// set a pattern for leds
fn main() -> ! {
//	    let vector = vec![1, 3, 4, 5, 3];  // cannot find macro `vec!` in this scope
        let _vc = [1,2,3,4,5];
        let _mx = _vc.iter().max().unwrap();


    if let Some(p) = Peripherals::take() {
		let gpio = p.GPIO.split();
		let tx_pin = gpio.pin24.into_push_pull_output().downgrade();
        let rx_pin = gpio.pin25.into_floating_input().downgrade();
		let (mut tx, _) = Serial::uart0(p.UART0, tx_pin, rx_pin, BAUD115200).split();
        write!(tx, "test output message\n\r").unwrap();
        let mut delay = Delay::new(p.TIMER0);
 //       let gpio = p.GPIO.split();
        let col1 = gpio.pin4.into_push_pull_output();
        let col2 = gpio.pin5.into_push_pull_output();
        let col3 = gpio.pin6.into_push_pull_output();
        let col4 = gpio.pin7.into_push_pull_output();
        let col5 = gpio.pin8.into_push_pull_output();
        let col6 = gpio.pin9.into_push_pull_output();
        let col7 = gpio.pin10.into_push_pull_output();
        let col8 = gpio.pin11.into_push_pull_output();
        let col9 = gpio.pin12.into_push_pull_output();
        let row1 = gpio.pin13.into_push_pull_output();
        let row2 = gpio.pin14.into_push_pull_output();
        let row3 = gpio.pin15.into_push_pull_output();
        let mut leds = Display::new(    // note 9 columns and 3 rows
            col1, col2, col3, col4, col5, col6, col7, col8, col9, row1, row2, row3,
        );
//https://www.mathworks.com/help/supportpkg/microbit/ref/5x5ledmatrix.html
//https://microbit.org/guide/hardware/leds/
        let checker_a = [
            [1, 0, 1, 0, 1],
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
        ];
// set another pattern for leds
        let checker_b = [
            [1, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 1, 0, 1, 1],
        ];

        loop {
            leds.display(&mut delay, checker_a, 1000);
            leds.display(&mut delay, checker_b, 1000);
        }
        let  mut j = 0 ;
     (0..5).flat_map(|x| x * 100 .. x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| j=x);    // try these on BBC build
    }
    panic!();
}
