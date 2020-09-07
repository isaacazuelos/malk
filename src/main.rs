//! Experiments with bare metal rust programming on an Pro Micro.

#![no_std]
#![no_main]
#![feature(llvm_asm)]
// We only need the `#[panic_handler]` lang item, the linter misses it.
#[allow(unused_imports)]
use panic_halt;

use pro_micro::prelude::*;
use sparkfun_pro_micro as pro_micro; // TODO: rename

#[no_mangle]
fn main() {
    let peripherals = pro_micro::Peripherals::take().unwrap();
    let mut pins = sparkfun_pro_micro::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
        peripherals.PORTE,
        peripherals.PORTF,
    );

    let mut rx_led = pins.led_rx.into_output(&mut pins.ddr);

    let top_switch = pins.d4.into_pull_up_input(&mut pins.ddr);
    let bottom_switch = pins.d5.into_pull_up_input(&mut pins.ddr);

    loop {
        if bottom_switch.is_low().void_unwrap() {
            rx_led.set_low().void_unwrap();
        } else if top_switch.is_low().void_unwrap() {
            rx_led.set_high().void_unwrap();
        } else {
            rx_led.set_high().void_unwrap();
            pro_micro::delay_ms(100);
            rx_led.set_low().void_unwrap();
            pro_micro::delay_ms(100);
        }
    }
}
