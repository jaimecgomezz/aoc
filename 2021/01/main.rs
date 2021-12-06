use std::fs;

pub fn main() {
    println!("{:?}", count(group(parse("input.txt"), 1))); // Part 1
    println!("{:?}", count(group(parse("input.txt"), 3))); // Part 2
}

fn parse(path: &str) -> Vec<usize> {
    let content = fs::read_to_string(path).unwrap();

    return content
        .lines()
        .into_iter()
        .map(|m| m.parse::<usize>().unwrap())
        .collect();
}

fn group(measurements: Vec<usize>, size: usize) -> Vec<usize> {
    if size < 2 {
        return measurements;
    }

    let mut result = vec![];

    let mut index = 0;
    let limit = measurements.len() - (size - 1);

    while index < limit {
        let mut sum = 0;

        for cindex in index..(index + size) {
            sum += measurements[cindex];
        }

        result.push(sum);

        index += 1;
    }

    return result;
}

fn count(measurements: Vec<usize>) -> usize {
    let mut result = 0;

    if measurements.is_empty() {
        return result;
    }

    let mut current: usize = measurements.first().unwrap().to_owned();

    for measurement in measurements {
        if measurement > current {
            result += 1;
        }

        current = measurement;
    }

    return result;
}
