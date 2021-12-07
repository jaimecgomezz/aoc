use std::{fs, io::BufRead};

pub fn main() {
    println!("{}", compute(count(parse("input.txt"))));
}

fn parse(path: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(path).unwrap();

    let length = content.len();
    let lines = content.clone().lines().count();
    let size = length / lines;

    let mut result: Vec<Vec<char>> = vec![vec![]; size - 1];

    for line in content.lines() {
        for (index, charr) in line.chars().enumerate() {
            let current = result.get_mut(index).unwrap();

            current.push(charr);
        }
    }

    return result;
}

fn count(input: Vec<Vec<char>>) -> Vec<isize> {
    let mut result: Vec<isize> = vec![0; input.len()];

    for (index, vector) in input.iter().enumerate() {
        for charr in vector {
            let current = result.get_mut(index).unwrap();

            *current += match charr {
                '1' => 1,
                '0' => -1,
                _ => 0,
            }
        }
    }

    return result;
}

fn compute(counts: Vec<isize>) -> usize {
    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for count in counts {
        if count > 0 {
            gamma = format!("{}{}", gamma, "1");
            epsilon = format!("{}{}", epsilon, "0");
        } else if count < 0 {
            gamma = format!("{}{}", gamma, "0");
            epsilon = format!("{}{}", epsilon, "1");
        }
    }

    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    return gamma * epsilon;
}
