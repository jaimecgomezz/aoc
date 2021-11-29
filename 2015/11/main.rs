use std::fs;

pub fn main() {
    let mut password = parse("input.txt");

    password = increase(password);

    while !valid(&password) {
        password = increase(password);
    }

    println!("{}", password);
}

fn parse(path: &str) -> String {
    let content = fs::read_to_string(path).unwrap();

    content.replace("\n", "")
}

fn increase(password: String) -> String {
    let mut result = String::from("");

    let mut chars: Vec<char> = password.chars().collect();

    while !chars.is_empty() {
        let last = chars.pop().unwrap();

        match last {
            'z' => {
                let next = 'a';

                result = format!("{}{}", next, result);
            }
            _ => {
                let next = ((last as u8) + (1 as u8)) as char;

                result = format!("{}{}", next, result);

                break;
            }
        }
    }

    let remaining: String = chars.into_iter().collect();

    result = format!("{}{}", remaining, result);

    return result;
}

fn valid(password: &String) -> bool {
    let mut items: Vec<u8> = password.chars().map(|c| c as u8).collect();

    r1(&items) && r2(&items) && r3(&items)
}

fn r1(items: &Vec<u8>) -> bool {
    let mut result = false;

    for index in 0..(items.len() - 2) {
        let current = items[index];
        let next = items[index + 1];
        let nextt = items[index + 2];

        if next == (current + 1) {
            if nextt == (next + 1) {
                result = true;
            }
        }
    }

    return result;
}

fn r2(items: &Vec<u8>) -> bool {
    let mut result = true;

    let prohibited: Vec<u8> = vec![105, 108, 111];

    for current in items {
        if prohibited.contains(&current) {
            result = false;
        }
    }

    return result;
}

fn r3(items: &Vec<u8>) -> bool {
    let mut result = false;

    let mut count = 0;
    let mut repeated = 0;

    for index in 0..(items.len() - 1) {
        let current = items[index];
        let next = items[index + 1];

        if current == next {
            if current != repeated {
                count += 1;
                repeated = current;
            }
        }
    }

    result = count == 2;

    return result;
}
