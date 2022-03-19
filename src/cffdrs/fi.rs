pub fn fi_calc(fc: &[f64], ros: &[f64]) -> Result<Vec<f64>, f64> {
    // Same length guard
    if fc.len() != ros.len() {
        return Err(-1.0);
    }
    let fi = fc
        .iter()
        .zip(ros.iter())
        .map(|(x, y)| 300.0 * x * y)
        .collect();
    return Ok(fi);
}
mod tests {
    use crate::cffdrs::fi::fi_calc;

    #[test]
    fn basic_single_input() {
        let result = fi_calc(&[1.0], &[1.0]);
        assert_eq!(result, Ok(vec![300.0]));
    }
    #[test]
    fn basic_error_wrong_lengths() {
        let result = fi_calc(&[1.0, 1.0], &[1.0]);
        assert_eq!(result, Err(-1.0));
    }
    #[test]
    fn basic_multiple_input() {
        let result = fi_calc(&[1.0, 1.0], &[1.0, 1.0]);
        assert_eq!(result, Ok(vec![300.0, 300.0]));
    }
}
