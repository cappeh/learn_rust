pub fn add_two(input: u64) -> u64 {
    input + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn check_add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    #[ignore]
    fn check_add_100_and_two() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}
