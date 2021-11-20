// cargo-deps: md5="0.7.0", regex="1.5"

extern crate md5;
extern crate regex;

use regex::Regex;
use std::{fmt::format, fs};

pub fn main() {
    let rg = Regex::new(r"^0{5}").unwrap();

    let key = fs::read_to_string("input.txt")
        .expect("Error reading from input")
        .replace("\n", "");

    let mut current = 0;
    loop {
        let formatted = format!("{}{}", key, current);

        let digest = md5::compute(formatted.as_bytes());
        let digest = format!("{:?}", digest);

        match rg.captures(digest.as_str()) {
            Some(_) => break,
            None => current += 1,
        }
    }

    println!("{}", current);
}
