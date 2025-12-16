#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use microbit::{Board, display::blocking::Display};

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use embedded_hal::digital::OutputPin;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut button_a = board.buttons.button_a;
    let mut button_state = false;

    loop {
        if button_a.is_low().unwrap() {
            if button_state == false {
                button_state = true;
                board.display_pins.col1.set_low().unwrap();
                board.display_pins.row1.set_high().unwrap();            }
        } else {
            if button_state == true {
                button_state = false;
                board.display_pins.col1.set_low().unwrap();
                board.display_pins.row1.set_low().unwrap();            }
            }
        }
    }




