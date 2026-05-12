#![no_std]
#![no_main]

mod states;
use panic_halt as _;
use crate::states::flight_state::FlightState;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    let mut tick: u32 = 0;
    let mut state = FlightState::Idle;

    loop {
        tick += 1;

        state = match state {
            FlightState::Idle => {
                if tick > 100 {
                    FlightState::Armed
                } else {
                    FlightState::Idle
                }
            }

            FlightState::Armed => {
                if tick > 200 {
                    FlightState::Ascent
                } else {
                    FlightState::Armed
                }
            }



            FlightState::Ascent => FlightState::Ascent,
            FlightState::Descent => FlightState::Descent,
        };

        match state {
            FlightState::Idle => led.set_low(),
            FlightState::Armed => led.toggle(),
            FlightState::Ascent => led.set_high(),
            FlightState::Descent => led.set_low(),
        }

        arduino_hal::delay_ms(50);
    }
}