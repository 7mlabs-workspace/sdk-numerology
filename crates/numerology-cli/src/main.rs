use std::io::{self, Read};
fn main() {
    let mut input = Vec::new();
    if io::stdin()
        .take((numerology_core::MAX_REQUEST_BYTES + 1) as u64)
        .read_to_end(&mut input)
        .is_err()
    {
        eprintln!("Unable to read input");
        std::process::exit(2)
    }
    println!("{}", numerology_core::calculate_bytes(&input));
}
