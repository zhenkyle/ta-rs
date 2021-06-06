#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max3() {
        assert_eq!(max3(3.0, 2.0, 1.0), 3.0);
        assert_eq!(max3(2.0, 3.0, 1.0), 3.0);
        assert_eq!(max3(2.0, 1.0, 3.0), 3.0);
    }
}
