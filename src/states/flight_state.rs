#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum FlightState {
    Idle,
    Armed,
    SimulatedAscent,
    Decent
}