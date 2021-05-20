
use restaurant::front_of_house::hosting::{waitlist, restaurant as imbiss};

fn main() {    
    println!("{}", waitlist::add_to());
    println!("{}", imbiss::leave())
}
