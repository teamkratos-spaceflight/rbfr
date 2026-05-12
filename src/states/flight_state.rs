#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum FlightState {
    Idle,
    Armed,
    Ascent,
    Descent
}