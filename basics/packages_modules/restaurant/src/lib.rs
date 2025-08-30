mod back_of_the_house {
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

pub fn eat_at_restaurant() {
    let mut meal = back_of_the_house::Breakfast::summer("rye");
    meal.toast = String::from("Wheat");
    println!("I'd like a {} toast please", meal.toast);
}


