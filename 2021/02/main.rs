use std::fs;

#[derive(Debug)]
struct Positionable {
    x: isize,
    y: isize,
    aim: isize,
}

impl Positionable {
    fn new() -> Self {
        Positionable { x: 0, y: 0, aim: 0 }
    }

    fn execute(&mut self, action: Action) {
        // Part 1
        // match action.instruction.as_str() {
        //     "up" => self.y -= action.units,
        //     "down" => self.y += action.units,
        //     "forward" => self.x += action.units,
        //     _ => (),
        // }

        // Part 2
        match action.instruction.as_str() {
            "up" => self.aim -= action.units,
            "down" => self.aim += action.units,
            "forward" => {
                self.x += action.units;
                self.y += self.aim * action.units;
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
struct Action {
    units: isize,
    instruction: String,
}

impl Action {
    fn from_line(line: &str) -> Self {
        let items: Vec<&str> = line.split(" ").collect();

        let units = items[1].parse::<isize>().unwrap();
        let instruction = items[0].to_string();

        Action { units, instruction }
    }
}

pub fn main() {
    let mut submarine = Positionable::new();

    println!("{:?}", compute(&mut submarine, parse("input.txt")));
}

fn parse(path: &str) -> Vec<Action> {
    let mut result: Vec<Action> = vec![];

    let content = fs::read_to_string(path).unwrap();

    for line in content.lines() {
        result.push(Action::from_line(line));
    }

    return result;
}

fn compute(positionable: &mut Positionable, actions: Vec<Action>) -> isize {
    for action in actions {
        positionable.execute(action);
    }

    return positionable.x * positionable.y;
}
