pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // this attribute indicates the following is a test function
    // test runner treats it as a test
    // may have some functions to setup some operation. the test attribute differentiates from
    // those functions
    #[test] 
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); // macro to assert the result
        // checks whether the two provided results are equal
    }

    #[test]
    fn another() {
        panic!("this test will fail");
    }
}

// use "cargo test" to run all tests in the project
