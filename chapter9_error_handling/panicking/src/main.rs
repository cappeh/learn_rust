fn drink(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("this is not your party. Run!!!");
        } else {
            println!("spit it out!!!");
        }
    } else {
        println!("some refreshing {} is all i need", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}
