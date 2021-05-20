pub mod front_of_house;

#[cfg(test)]
mod tests {

    use super::front_of_house::hosting::table;
    use super::front_of_house::hosting::restaurant;
    use super::front_of_house::hosting::waitlist;
    use super::front_of_house::serving::serving;

    #[test]
    fn test_seat_at_table() {
        assert_eq!(table::seat_at(), "seat_at_table");
    }

    #[test]
    fn test_leave_restaurent() {
        assert_eq!(restaurant::leave(), "leave_restaurant");
    }

    #[test]
    fn test_add_to_waitlist() {
        assert_eq!(waitlist::add_to(), "add_to_waitlist");
    }

    #[test]
    fn test_take_order() {
        assert_eq!(serving::take_order(), "take_order");
    }
}
