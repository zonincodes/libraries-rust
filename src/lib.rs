pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {

    // aboslute path
    hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String, 
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn a_test() {
        let name: usize = 2;
        assert_eq!(name, 2);
    }

    #[test]
    fn all_tests(){
        assert_eq!(front_of_house::hosting::add_to_waitlist(), ());
        assert_eq!(front_of_house::hosting::seat_at_table(), "Seat");
        assert_eq!(front_of_house::serving::serve_order(), 4);
        assert_eq!(front_of_house::serving::take_order(), 3);
        assert_eq!(front_of_house::serving::take_payment(), 6);
    }

    #[test]
    fn struct_test() {
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");

        assert_ne!(meal.toast, "Rye");
    }
}
