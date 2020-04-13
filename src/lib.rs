pub fn celsius_to_fahrenheit(degree : f32) -> f32 {
    return (degree * (9.0 / 5.0)) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_degree_celsius() {
        assert_eq!(32.0, celsius_to_fahrenheit(0.0));
    }
}
