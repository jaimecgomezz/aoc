// cargo-deps: fancy-regex="0.7.1"

extern crate fancy_regex;

use fancy_regex::Regex;
use std::fmt::format;
use std::fs;

pub fn main() {
    let mut count = 0;
    let content = fs::read_to_string("input.txt").expect("Error reading input");

    let rg1 = Regex::new(r"\w*(\w{2})\w*\1\w*").unwrap();
    let rg2 = Regex::new(r"(\w).\1").unwrap();

    for line in content.lines() {
        let mut score = 0;

        match rg1.captures(line).unwrap() {
            Some(_) => score += 1,
            _ => (),
        }

        match rg2.captures(line).unwrap() {
            Some(_) => score += 1,
            _ => (),
        }

        if score == 2 {
            count += 1;
        }
    }

    println!("{}", count);
}
