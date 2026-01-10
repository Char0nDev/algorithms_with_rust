// Trapezoidal rule
pub fn area_under_curve(start: f64, end: f64, func: fn(f64) -> f64, step_count: usize) -> f64 {
    assert!(step_count > 0);

    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };

    let step_length: f64 = (end - start) / step_count as f64;
    let mut area = 0f64;
    let mut fx1 = func(start);
    let mut fx2: f64;

    // (1..=step_count).map(|x| (x as f64 * step_length) + start) :  the values of x

    for point in (1..=step_count).map(|x| (x as f64 * step_length) + start) {
        fx2 = func(point);
        area += (fx1 + fx2).abs() * step_length * 0.5;
        // (fx1 + fx2) to do y_0 + 2y_1 + 2y_2 +.... y_n
        fx1 = fx2
    }

    area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_func() {
        assert_eq!(area_under_curve(1f64, 2f64, |x| x, 10), 1.5000000000000002)
    }

    #[test]
    fn test_zero_length() {
        assert_eq!(area_under_curve(0f64, 0f64, |x| x * x, 1000), 0.0);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            area_under_curve(1f64, 2f64, |x| x, 10),
            area_under_curve(2f64, 1f64, |x| x, 10)
        );
    }
}
