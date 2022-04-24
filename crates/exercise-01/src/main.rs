use std::fs::File;
use std::io::{BufRead, BufReader};
pub mod lib;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        match line.parse::<lib::Book>() {
            Ok(my_book) => println!("{}", my_book),
            Err(_) => continue,
        };
    }
}
