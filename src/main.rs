use std::env;

mod dhash;

fn main() {
    let filename = env::args().nth(1).expect("Need a filename");
    println!("{}: {}", dhash::dhash(&filename).unwrap_or("FAILURE".to_string()), filename);
}
