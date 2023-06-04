fn main() {
    make_coffee(2, "Mocha");
}

fn make_coffee(num_of_cups: u32, flavor: &str) {
    println!("You asked for {} cups of {} coffee.", num_of_cups, flavor);
}
