use std::fs;

pub fn main() {
    let content = fs::read_to_string("input.txt").expect("Input error.");

    let mut absolute_real = 0;
    let mut absolute_total = 0;

    for line in content.lines() {
        let mut real = 0;
        let total = line.len();

        let chars = line.chars().collect::<Vec<char>>();

        let mut index = 1;
        loop {
            match chars[index] {
                '\\' => match chars[index + 1] {
                    '\\' | '"' => index += 2,
                    'x' => index += 4,
                    _ => index += 1,
                },
                _ => index += 1,
            }

            real += 1;

            if index >= (total - 1) {
                break;
            }
        }

        absolute_real += real;
        absolute_total += total;
    }

    println!("{}", absolute_total - absolute_real);
}
