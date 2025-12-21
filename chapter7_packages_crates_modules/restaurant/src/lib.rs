#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

#[allow(dead_code)]
fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{meal:?}");
}

#[allow(dead_code)]
fn deliver_order() {}

mod back_of_house {
    #[allow(dead_code)]
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }
}

