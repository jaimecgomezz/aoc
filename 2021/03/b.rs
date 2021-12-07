use std::cmp::Ordering;
use std::fs;
use std::ops::Not;

#[derive(Debug)]
enum Criteria {
    Most,
    Least,
}

pub fn main() {
    println!("{:?}", compute(&mut parse("input.txt")));
}

fn parse(path: &str) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![];

    let content = fs::read_to_string(path).unwrap();

    for line in content.lines() {
        let charss: Vec<char> = line.chars().collect();

        let parsed: Vec<u32> = charss
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        result.push(parsed);
    }

    return result;
}

fn compute(matrix: &mut Vec<Vec<u32>>) -> usize {
    let mut oxy = reduce(&mut matrix.to_owned(), Criteria::Most);
    let mut co2 = reduce(&mut matrix.to_owned(), Criteria::Least);

    return oxy * co2;
}

fn reduce(matrix: &mut Vec<Vec<u32>>, criteria: Criteria) -> usize {
    if matrix.is_empty() {
        return 0;
    }

    let mut column = 0;
    let climit = matrix.first().unwrap().len();

    while matrix.len() > 1 {
        if column == climit {
            column = 0;
        }

        let mut ones = 0;

        for line in 0..matrix.len() {
            ones += matrix[line][column];
        }

        let zeros = matrix.len() as u32 - ones;

        let target: u32 = match ones.cmp(&zeros) {
            Ordering::Less => match criteria {
                Criteria::Most => 1,
                Criteria::Least => 0,
            },
            Ordering::Greater => match criteria {
                Criteria::Most => 0,
                Criteria::Least => 1,
            },
            Ordering::Equal => match criteria {
                Criteria::Most => 0,
                Criteria::Least => 1,
            },
        };

        filter(matrix, column.to_owned(), target);

        column += 1;
    }

    return transform(matrix.first());
}

fn filter(matrix: &mut Vec<Vec<u32>>, column: usize, target: u32) {
    matrix.retain(|vector| vector[column] != target);
}

fn transform(first: Option<&Vec<u32>>) -> usize {
    match first {
        Some(vec) => {
            let mut formated = String::from("");

            for digit in vec {
                formated = format!("{}{}", formated, digit);
            }

            usize::from_str_radix(&formated, 2).unwrap()
        }
        None => 0,
    }
}
