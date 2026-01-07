pub fn add_two(input: u64) -> u64 {
    input + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
