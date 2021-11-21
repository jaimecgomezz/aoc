use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_coordinated(coordinates: &str) -> Self {
        let coordinates: Vec<&str> = coordinates.split(",").collect();

        let x: usize = coordinates[0].parse::<usize>().unwrap();
        let y: usize = coordinates[1].parse::<usize>().unwrap();

        Point { x, y }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Error reading input.");

    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for line in content.lines() {
        let items: Vec<&str> = line.split(" ").collect();

        match items[0] {
            "turn" => {
                let state = match items[1] {
                    "on" => 1,
                    _ => 0,
                };

                let lb = Point::from_coordinated(items[2]);
                let rt = Point::from_coordinated(items[4]);

                turn(state, &lb, &rt, &mut grid)
            }
            "toggle" => {
                let lb = Point::from_coordinated(items[1]);
                let rt = Point::from_coordinated(items[3]);

                toogle(&lb, &rt, &mut grid)
            }
            _ => continue,
        }
    }

    println!("{}", sum(grid));
}

fn turn(state: usize, lb: &Point, rt: &Point, grid: &mut Vec<Vec<usize>>) {
    for y in lb.y..=rt.y {
        for x in lb.x..=rt.x {
            grid[y][x] = state;
        }
    }
}

fn toogle(lb: &Point, rt: &Point, grid: &mut Vec<Vec<usize>>) {
    for y in lb.y..=rt.y {
        for x in lb.x..=rt.x {
            grid[y][x] = if grid[y][x] == 1 { 0 } else { 1 };
        }
    }
}

fn sum(grid: Vec<Vec<usize>>) -> usize {
    let mut result = 0;

    for row in grid {
        let presult = match row.into_iter().reduce(|a, b| a + b) {
            Some(r) => r,
            None => 0,
        };

        result += presult;
    }

    return result;
}
