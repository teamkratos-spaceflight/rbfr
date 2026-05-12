use super::imu::Imu;


pub struct FakeImu {
    tick: u32,
    altitude: f32,
    velocity: f32,
}

impl FakeImu {
    pub fn new() -> Self {
        Self { tick: 0, altitude: 0.0, velocity: 0.0 }
    }
}

impl Imu for FakeImu {
    fn accel(&mut self) -> (f32, f32, f32) {
        self.tick += 1;

        // fake thrust phase then gravity
        let thrust = if self.tick < 100 { 1.5 } else { -1.0 };

        self.velocity += thrust * 0.05;
        self.altitude += self.velocity;

        (0.0, 0.0, thrust + 1.0) // include gravity baseline
    }

    fn altitude(&mut self) -> f32 {
        self.altitude
    }
}