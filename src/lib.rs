pub fn almost_equal(left: f64, right: f64) -> bool {
    (left - right).abs() < f64::EPSILON
}

#[cfg(test)]
mod test {
    #[test]
    fn almost_equal() {
        assert!(super::almost_equal(0.1 + 0.2, 0.3))
    }
}
