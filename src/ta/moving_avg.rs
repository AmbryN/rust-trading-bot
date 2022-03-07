/**
 * * Calculate Simple Average
 * @param values: [f32]
 * @return average: f32
 */
pub fn calc_simple_average(values: &[f32]) -> f32 {
    let mut sum: f32 = 0.;
    for &value in values {
        sum += value;
    }
    let average = sum / (values.len() as f32);
    average
}

/**
 * * Calculate Exponential Moving Average (EMA)
 * @param values: [f32]
 * @return expAverage: f32
 */
fn calc_exp_average(values: &[f32], lookback: usize) -> f32 {
    let multiplier = 2.0 / (lookback + 1) as f32;

    if values.len() == lookback {
        calc_simple_average(&values)
    } else {
        let next_considered_values = values.clone().get(0..values.len() - 1).unwrap_or_default();
        let previous_exp_avg = calc_exp_average(&next_considered_values, lookback);
        let exp_avg: f32 =
            (values.last().unwrap_or(&0.) - previous_exp_avg) * multiplier + previous_exp_avg;

        exp_avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_moving_avg_with_correct_values() {
        let values = [1., 2., 3., 4., 5., 6.];
        let result = calc_simple_average(&values);

        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_exp_moving_avg_with_correct_values() {
        let values = [1., 2., 3., 4., 5.];
        let result = calc_exp_average(&values, 3);

        assert_eq!(result, 4.);
    }
}
