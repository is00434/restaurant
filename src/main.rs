
mod front_of_house;

fn main() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::serve_order()
}