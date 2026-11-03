use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    if io::stdin()
        .take((numerology_core::MAX_REQUEST_BYTES + 1) as u64)
        .read_to_string(&mut input)
        .is_err()
    {
        eprintln!("Input must be UTF-8");
        std::process::exit(2)
    }
    println!("{}", numerology_core::calculate_json(&input));
}
