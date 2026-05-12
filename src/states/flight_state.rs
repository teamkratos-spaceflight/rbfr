#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlightState {
    Idle,
    Armed,
    Ascent,
    Descent
}