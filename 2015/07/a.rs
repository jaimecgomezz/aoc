use std::collections::HashMap;
use std::fs;

pub fn main() {
    let content = fs::read_to_string("input.txt").expect("Input error.");

    let mut wires: HashMap<&str, u16> = HashMap::new();
    let mut lines: Vec<&str> = content.lines().collect();

    let mut index = 0;
    while lines.len() > 0 {
        if index == lines.len() {
            index = 0;
        }

        let items: Vec<&str> = lines[index].split(" ").collect();

        let success = match items.len() {
            3 => {
                let source = items[0];
                let target = items[2];

                assign(source, target, &mut wires)
            }
            4 => {
                let source = items[1];
                let target = items[3];

                not(source, target, &mut wires)
            }
            5 => {
                let a = items[0];
                let b = items[2];
                let target = items[4];
                let operation = items[1];

                execute(operation, a, b, target, &mut wires)
            }
            _ => false,
        };

        if success {
            lines.remove(index);
        } else {
            index += 1;
        }
    }

    println!("{:?}", wires.get("a").unwrap());
}

// ref:1
fn assign<'a>(source: &str, target: &'a str, wires: &mut HashMap<&'a str, u16>) -> bool {
    let amount = match source.parse::<u16>() {
        Ok(parsed) => parsed,
        Err(_) => match wires.get(source) {
            Some(found) => found.to_owned(),
            None => return false,
        },
    };

    let current = wires.entry(target).or_insert(amount);
    *current = amount;

    return true;
}

// ref:1
fn not<'a>(source: &str, target: &'a str, wires: &mut HashMap<&'a str, u16>) -> bool {
    let amount = match wires.get(source) {
        Some(found) => !found.to_owned(),
        None => return false,
    };

    let current = wires.entry(target).or_insert(amount);
    *current = amount;

    return true;
}

// ref:1
fn execute<'a>(
    operation: &str,
    a: &str,
    b: &str,
    target: &'a str,
    wires: &mut HashMap<&'a str, u16>,
) -> bool {
    let a = match a.parse::<u16>() {
        Ok(parsed) => parsed,
        Err(_) => match wires.get(a) {
            Some(found) => found.to_owned(),
            None => return false,
        },
    };

    let b = match b.parse::<u16>() {
        Ok(parsed) => parsed,
        Err(_) => match wires.get(b) {
            Some(found) => found.to_owned(),
            None => return false,
        },
    };

    let amount = match operation {
        "OR" => a | b,
        "AND" => a & b,
        "LSHIFT" => a << b,
        "RSHIFT" => a >> b,
        _ => return false,
    };

    let current = wires.entry(target).or_insert(amount);
    *current = amount;

    return true;
}

// ref:1 https://users.rust-lang.org/t/lifetime-error-when-inserting-a-value-into-a-hashmap/27769
