mod simulator;

use simulator::Simulator;

use process_calibration::process_calibration;
use std::time::{Duration, Instant};
use std::thread;


fn main() {
    println!("Starting calibration simulation...");
    let mut simulator = Simulator::new();
    let mut samples = Vec::new();
    let start = Instant::now();
    
    while start.elapsed() < Duration::from_secs(10) {
        let t = start.elapsed().as_secs_f32();
        simulator.x_pos = 50.0 + 20.0 * f32::sin(t * 2.0);
        
        let reading = simulator.simulate_sensor_position();
        samples.push(reading.clone());
        println!("Sample: {:?}", reading);
        
        thread::sleep(Duration::from_millis(500));
    }
    
    match process_calibration(&samples, simulator::NUM_SENSORS) {
        Ok((min_values, max_values, thresholds)) => {
            println!("\nCalibration results:");
            println!("Min values:    {:?}", min_values);
            println!("Max values:    {:?}", max_values);
            println!("Thresholds:    {:?}", thresholds);
        },
        Err(e) => {
            println!("Calibration failed: {:?}", e);
        }
    }
}