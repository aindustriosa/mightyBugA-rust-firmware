use std::time::SystemTime;
use rand::Rng;

// Constants for simulation
pub const NUM_SENSORS: usize = 8;
pub const WHITE_VALUE: i32 = 2800;
pub const BLACK_VALUE: i32 = 200;
pub const TRANSITION_VALUE: i32 = 1500;
pub const SAMPLE_TIME_MS: u64 = 500;

/// Represents the line sensor simulator
pub struct Simulator {
    /// Current X position of robot
    pub x_pos: f32,
    /// Position of the line center
    pub line_center: f32,
    /// Space between sensors in units
    sensor_spacing: f32,
}

impl Simulator {
    /// Creates a new simulator with default values
    pub fn new() -> Self {
        Self {
            x_pos: 50.0,
            line_center: 50.0,
            sensor_spacing: 2.0,
        }
    }

    /// Adds random noise to sensor readings
    fn add_noise(value: i32) -> i32 {
        let noise = rand::thread_rng().gen_range(-50..=50);
        (value + noise).clamp(0, 3000)
    }

    /// Simulates readings from all sensors
    pub fn simulate_sensor_position(&self) -> Vec<i32> {
        let mut readings = Vec::with_capacity(NUM_SENSORS);

        for i in 0..NUM_SENSORS {
            // Calculate position for each sensor
            let sensor_pos = self.x_pos + (i as f32 - NUM_SENSORS as f32/2.0) * self.sensor_spacing;
            let distance = (sensor_pos - self.line_center).abs();
            
            // Determine sensor value based on distance from line
            let raw_value = if distance < 2.0 {
                BLACK_VALUE
            } else if distance < 4.0 {
                TRANSITION_VALUE
            } else {
                WHITE_VALUE
            };
            
            readings.push(Self::add_noise(raw_value));
        }

        readings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulator_new() {
        let sim = Simulator::new();
        assert_eq!(sim.x_pos, 50.0);
        assert_eq!(sim.line_center, 50.0);
    }

    #[test]
    fn test_sensor_readings() {
        let sim = Simulator::new();
        let readings = sim.simulate_sensor_position();
        assert_eq!(readings.len(), NUM_SENSORS);
    }
}