use std::net::IpAddr;

fn main() {
    // ip addr string is hardcoded here and we can tell that it is valid
    // so expect is appropriate as we dont expect this code to fail
    // if the source of input changed and an error could occur, we should use more robust
    // error-handling
    let _home: IpAddr = "127.0.0.1".parse()
        .expect("hardcoded IP address should be valid");
}
