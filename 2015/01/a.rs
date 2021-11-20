use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Problem while reading from file.");

    let mut level = 0;

    for c in content.chars() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => (),
        }
    }

    println!("{}", level);
}
