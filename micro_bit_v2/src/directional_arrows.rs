#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use microbit::{hal::gpio, Board};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Configure buttons
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let mut display = Display::new(board.display_pins);


    let left_arrow = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
    ];


      let right_arrow = [
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    let center_dot = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];



    loop {
        let on_pressed = button_a.is_low().unwrap();
        let off_pressed = button_b.is_low().unwrap();
        match (on_pressed, off_pressed) {
            // Stay in current state until something is pressed.
            (false, false) => (),
            // Change to on state.
            (true, false) =>  display.show(&mut timer, left_arrow, 1000),
            // Change to off state.
            (false, true) => display.show(&mut timer, right_arrow, 1000),
            // Stay in current state until something is released.
            (true, true) => display.show(&mut timer, center_dot, 1000),
        }
        timer.delay_ms(10_u32);
    }
}