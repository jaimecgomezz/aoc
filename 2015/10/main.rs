use std::fs;

pub fn main() {
    let cicles = 50;

    let mut input = parse("input.txt");

    for x in 0..cicles {
        input = recompose(decompose(input.to_owned()));
    }

    println!("{}", input.len());
}

fn parse(path: &str) -> String {
    let content = fs::read_to_string(path).unwrap();

    content.replace("\n", "")
}

fn decompose(input: String) -> Vec<(String, usize)> {
    let mut result: Vec<(String, usize)> = vec![];

    let mut count = 0;
    let mut current = input.chars().nth(0).unwrap();

    for c in input.chars() {
        // TODO: implement match
        if c == current {
            count += 1;
        } else {
            result.push((current.to_string(), count.to_owned()));

            current = c;
            count = 1;
        }
    }

    // account for the last repeating char
    result.push((current.to_string(), count.to_owned()));

    return result;
}

fn recompose(decomposed: Vec<(String, usize)>) -> String {
    let mut result = String::from("");

    for (c, count) in decomposed {
        let formatted = format!("{}{}", count, c);

        result.push_str(formatted.as_str());
    }

    return result;
}
