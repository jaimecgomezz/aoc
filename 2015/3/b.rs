use std::fmt;
use std::fs;
use std::ops::Not;

#[derive(Debug)]
struct Movable {
    x: isize,
    y: isize,
    name: String,
}

impl Movable {
    fn new(name: String) -> Self {
        Self { x: 0, y: 0, name }
    }

    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }
}

pub fn main() {
    let content = fs::read_to_string("input.txt").expect("Error reading file");

    let mut bot = Movable::new(String::from("bot"));
    let mut santa = Movable::new(String::from("santa"));

    let mut current: &mut Movable = &mut santa;
    let mut positions: Vec<isize> = vec![pair(current.x, current.y)];

    for c in content.chars() {
        match c {
            '^' => current.up(),
            '>' => current.right(),
            'v' => current.down(),
            '<' => current.left(),
            _ => (),
        }

        let paired = pair(current.x, current.y);

        if positions.contains(&paired).not() {
            positions.push(paired);
        }

        match current.name.as_str() {
            "bot" => current = &mut santa,
            "santa" => current = &mut bot,
            _ => (),
        }
    }

    println!("{}", positions.len());
}

// ref:1
fn pair(x: isize, y: isize) -> isize {
    let xx = if x >= 0 { x * 2 } else { x * -2 - 1 };
    let yy = if y >= 0 { y * 2 } else { y * -2 - 1 };

    if xx >= yy {
        xx * xx + xx + yy
    } else {
        yy * yy + xx
    }
}

// ref:1 https://www.vertexfragment.com/ramblings/cantor-szudzik-pairing-functions/
