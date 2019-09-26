#![no_std]
#![no_main]

#[macro_use(entry, exception)]
extern crate microbit;
extern crate cortex_m;
extern crate panic_abort;
extern crate cortex_m_rt;

use cortex_m_rt::ExceptionFrame;

exception!(*, default_handler);

fn default_handler(_irqn: i16) {}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

entry!(main_loop);

fn main_loop() -> ! {
    // OK, this is a very very optimistic code here.
    let p = microbit::Peripherals::take().take().unwrap();

    // configuring GPIO  pin P0.4 and P0.13 as output
    // to control the matrix LED point (COL1, ROW1)
    p.GPIO.pin_cnf[4].write(|w| w.dir().output());
    p.GPIO.pin_cnf[13].write(|w| w.dir().output());
    p.GPIO.out.write(|w| unsafe { w.bits(1 << 13) });

    let mut state: bool = true;
    loop {
        // some simple delay with busy waiting
        for _ in 0..1000 {
            cortex_m::asm::nop();
        }

        // toggling the state variable that represents the blinking
        state = !state;

        if state {
            // turn the LED on, but setting P0.4 to LOW and PO.13 to HIGH
            p.GPIO.out.write(|w| unsafe { w.bits(1 << 13) });
        } else {
            // turn the LED off by setting both GPIO pins to LOW.
            p.GPIO.out.write(|w| unsafe { w.bits(0) });
        }
    }
}
