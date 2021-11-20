use std::fs;

fn usized(number: &str) -> usize {
    number.to_string().parse::<usize>().unwrap()
}

pub fn main() {
    let content = fs::read_to_string("input.txt").expect("Couldn't read from file.");

    let mut result = 0;

    for line in content.lines() {
        let mut dims: Vec<usize> = line.split("x").map(usized).collect();

        dims.sort();

        result += ((dims[0] * 2) + (dims[1] * 2)) + (dims[0] * dims[1] * dims[2]);
    }

    println!("{}", result);
}
