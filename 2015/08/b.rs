use std::fs;

pub fn main() {
    let content = fs::read_to_string("input.txt").expect("Input error.");

    let mut absolute_total = 0;
    let mut absolute_escaped = 0;

    for line in content.lines() {
        let total = line.len();

        let mut escaped = 4;

        let chars = line.chars().collect::<Vec<char>>();

        let mut index = 1;
        loop {
            match chars[index] {
                '\\' => match chars[index + 1] {
                    '\\' | '"' => {
                        index += 2;
                        escaped += 2;
                    }
                    'x' => {
                        index += 4;
                        escaped += 1;
                    }
                    _ => index += 1,
                },
                _ => index += 1,
            }

            if index >= (total - 1) {
                break;
            }
        }

        absolute_total += total;
        absolute_escaped += total + escaped;
    }

    println!("{}", absolute_escaped - absolute_total);
}
