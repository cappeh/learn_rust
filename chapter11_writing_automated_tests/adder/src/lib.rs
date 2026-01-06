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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); // macro to assert the result
        // checks whether the two provided results are equal
    }
}

// use "cargo test" to run all tests in the project
