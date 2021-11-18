use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Problem while reading from file.");

    let mut level = 0;
    let mut position = 0;

    for (n, c) in content.chars().enumerate() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => (),
        }

        if level < 0 {
            position = n + 1;
            break;
        }
    }

    println!("{}", position);
}
