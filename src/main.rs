#![no_std]
#![no_main]

mod states;
mod sensors;

use panic_halt as _;
use crate::states::flight_state::FlightState;
use crate::sensors::fake_imu::FakeImu;
use crate::sensors::imu::Imu;
// for testing

const LAUNCH_THRESHOLD: f32 = 5.0;
const APOGEE_CONFIRMATION: u8 = 5;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();
    let mut imu: FakeImu = FakeImu::new();

    let mut state_tick: u32 = 0;
    let mut tick: u32 = 0;
    let mut state = FlightState::Idle;

    let mut last_altitude: f32 = 0.0;
    let mut falling_counter: u8 = 0;

    fn log<S: ufmt::uWrite>(
        serial: &mut S,
        tick: u32,
        state: &FlightState,
        alt: f32,
        ax: f32,
        ay: f32,
        az: f32,
    ) {
        let _ = ufmt::uwrite!(serial, "{},", tick);
        match state {
            FlightState::Idle => { let _ = ufmt::uwrite!(serial, "Idle"); },
            FlightState::Armed => { let _ = ufmt::uwrite!(serial, "Armed"); },
            FlightState::Ascent => { let _ = ufmt::uwrite!(serial, "Ascent"); },
            FlightState::Descent => { let _ = ufmt::uwrite!(serial, "Descent"); },
        }

        fn print_float<S: ufmt::uWrite>(serial: &mut S, f: f32) {
            let i = f as i32;
            let d = ((f - i as f32) * 100.0).abs() as i32;
            let _ = ufmt::uwrite!(serial, ",{}.", i);
            if d < 10 {
                let _ = ufmt::uwrite!(serial, "0");
            }
            let _ = ufmt::uwrite!(serial, "{}", d);
        }

        print_float(serial, alt);
        print_float(serial, ax);
        print_float(serial, ay);
        print_float(serial, az);
        let _ = ufmt::uwrite!(serial, "\r\n");
    }


    loop {
        tick += 1;
        state_tick += 1;

        let (_ax, _ay, az) = FakeImu::accel(&mut imu);
        let launch_detected = az > LAUNCH_THRESHOLD;
        let altitude = FakeImu::altitude(&mut imu);

        let rising = altitude > last_altitude;
        log(&mut serial, tick, &state, altitude, _ax, _ay, az);

        if rising {
            falling_counter = 0;
        } else {
            falling_counter += 1;
        }

        last_altitude = altitude;

        if launch_detected {
            // launch detection logic could be expanded here
        }

        let new_state = match state {

            FlightState::Idle => {
                if launch_detected {
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

            FlightState::Ascent => {
                if falling_counter > APOGEE_CONFIRMATION {
                    state_tick = 0;
                    FlightState::Descent
                } else {
                    FlightState::Ascent
                }
            }
            FlightState::Descent => FlightState::Descent,

        };

        state = new_state;

        match state {
            FlightState::Idle => led.set_low(),
            FlightState::Armed => led.set_high(), // simple indication
            FlightState::Ascent => led.set_high(),
            FlightState::Descent => led.set_low(),
        }
    }
}
