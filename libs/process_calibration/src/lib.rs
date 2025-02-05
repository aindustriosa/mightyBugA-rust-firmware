#[derive(Debug)]
pub enum CalibrationError {
    InvalidSensorCount,
    EmptySamples,
    InconsistentSamples,
}

pub fn process_calibration(samples: &[Vec<i32>], num_sensors: usize) 
    -> Result<(Vec<i32>, Vec<i32>, Vec<i32>), CalibrationError> {
    
    if samples.is_empty() {
        return Err(CalibrationError::EmptySamples);
    }

    if samples.iter().any(|s| s.len() != num_sensors) {
        return Err(CalibrationError::InconsistentSamples);
    }
    
    let mut min_values = vec![i32::MAX; num_sensors];
    let mut max_values = vec![i32::MIN; num_sensors];
    
    for sample in samples {
        for (i, &value) in sample.iter().enumerate() {
            min_values[i] = min_values[i].min(value);
            max_values[i] = max_values[i].max(value);
        }
    }
    
    let thresholds: Vec<i32> = min_values.iter()
        .zip(max_values.iter())
        .map(|(&min, &max)| (min + max) / 2)
        .collect();
    
    Ok((min_values, max_values, thresholds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_calibration() {
        let samples = vec![
            vec![100, 200],
            vec![150, 250],
        ];
        let result = process_calibration(&samples, 2).unwrap();
        assert_eq!(result.0, vec![100, 200]); // min
        assert_eq!(result.1, vec![150, 250]); // max
        assert_eq!(result.2, vec![125, 225]); // thresholds
    }

    #[test]
    fn test_empty_samples() {
        let samples: Vec<Vec<i32>> = vec![];
        assert!(matches!(
            process_calibration(&samples, 2),
            Err(CalibrationError::EmptySamples)
        ));
    }

    #[test]
    fn test_inconsistent_samples() {
        let samples = vec![
            vec![100, 200],
            vec![150],
        ];
        assert!(matches!(
            process_calibration(&samples, 2),
            Err(CalibrationError::InconsistentSamples)
        ));
    }
}