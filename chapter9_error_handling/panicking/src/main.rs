// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         // ################################################################
//         // in the Cargo.toml file add the following:
//         // [profile.dev] ======================= or [profile.release]
//         // panic = "abort"
//         // ################################################################
//         // RUSTFLAGS="-C panic=abort" cargo run (cargo build --release)
//         if cfg!(panic = "abort") {
//             println!("this is not your party. Run!!!");
//         } else {
//             println!("spit it out!!!");
//         }
//     } else {
//         println!("some refreshing {} is all i need", beverage);
//     }
// }

// this function runs when the panic strategy is the default "unwind"
#[cfg(panic = "unwind")]
fn ah() {
    println!("spit it out!!!");
}

// if the panic strategy is not "unwind" and changed to abort, this function is called
#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("this is not your party. Run!!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("some refreshing {} is all i need", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}
