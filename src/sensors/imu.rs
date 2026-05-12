pub trait Imu {
    fn accel(&mut self) -> (f32, f32, f32);
}