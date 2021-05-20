pub mod front_of_house;

#[cfg(test)]
mod tests {

    use super::front_of_house::hosting::seat_at_table;
    use super::front_of_house::hosting::leave_restaurent;
    use super::front_of_house::hosting::add_to_waitlist;
    use super::front_of_house::serving::serving;

    #[test]
    fn test_seat_at_table() {
        assert_eq!(seat_at_table::seat_at_table(), "seat_at_table");
    }

    #[test]
    fn test_leave_restaurent() {
        assert_eq!(leave_restaurent::leave_restaurent(), "leave_restaurent");
    }

    #[test]
    fn test_add_to_waitlist() {
        assert_eq!(add_to_waitlist::add_to_waitlist(), "add_to_waitlist");
    }

    #[test]
    fn test_take_order() {
        assert_eq!(serving::take_order(), "take_order");
    }
}
