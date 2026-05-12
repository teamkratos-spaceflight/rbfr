#[derive(Copy, Clone)]
enum FlightState {
    Idle,
    Armed,
    SimulatedAscent,
    Decent
}