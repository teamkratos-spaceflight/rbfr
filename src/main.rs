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

    let mut state_tick: u32 = 0;
    let mut tick: u32 = 0;
    let mut state = FlightState::Idle;

    loop {
        tick += 1;
        state_tick += 1;

        let new_state = match state {
            FlightState::Idle => {
                if state_tick > 100 {
                    state_tick = 0;
                    FlightState::Armed
                } else {
                    FlightState::Idle
                }
            }

            FlightState::Armed => {
                if state_tick > 100 {
                    state_tick = 0;
                    FlightState::Ascent
                } else {
                    FlightState::Armed
                }
            }

            FlightState::Ascent => FlightState::Ascent,
            FlightState::Descent => FlightState::Descent,
        };

        state = new_state;
    }
}
