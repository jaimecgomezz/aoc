// cargo-deps: fancy-regex="0.7.1"

extern crate fancy_regex;

use fancy_regex::Regex;
use std::fs;

pub fn main() {
    let mut count = 0;
    let content = fs::read_to_string("input.txt").expect("Error reading input");

    let rg1 = Regex::new(r"\w*[aeiou]\w*[aeiou]\w*[aeiou]\w*").unwrap();
    let rg2 = Regex::new(r"(\w)\1+").unwrap();
    let rg3 = Regex::new(r"\w*(ab|cd|pq|xy)\w*").unwrap();

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

        match rg3.captures(line).unwrap() {
            None => score += 1,
            _ => (),
        }

        if score == 3 {
            count += 1;
        }
    }

    println!("{}", count);
}
