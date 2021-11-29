// cargo-deps: regex = "1.5.4"

extern crate regex;

use regex::Regex;
use std::fs;

pub fn main() {
    let input = parse("input.txt");

    let mut result = 0;
    let re = Regex::new(r"([-]*\d+)").unwrap();

    for capture in re.captures_iter(input.as_str()) {
        let plain: &str = capture.get(1).unwrap().as_str();
        let number = plain.parse::<isize>().unwrap();

        result += number;
    }

    println!("{}", result);
}

fn parse(path: &str) -> String {
    let result = fs::read_to_string(path).unwrap();

    result.replace("\n", "")
}
