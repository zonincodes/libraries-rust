pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() -> String { let str: String = String::from("Seat"); return str; }
    }

    pub mod serving {
        pub fn take_order() -> u32 { 3 }
        pub fn serve_order() -> u32 { 4 }
        pub fn take_payment() -> i32 { 6 }
    }
}

pub fn eat_at_restaurant() {

    // aboslute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
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

}
