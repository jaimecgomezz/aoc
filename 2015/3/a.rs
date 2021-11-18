use std::{fs, ops::Not};

pub fn main() {
    let mut content = fs::read_to_string("input.txt").expect("File!");

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut positions: Vec<isize> = vec![pair(x, y)];

    for c in content.chars() {
        match c {
            '^' => y += 1,
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            _ => (),
        }

        let paired = pair(x, y);

        if positions.contains(&paired).not() {
            positions.push(paired);
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

// ref:1
// unnecessary but cool to implemnt, lol
fn unpair(z: isize) -> (isize, isize) {
    let zz = z as f64;

    let sqrtz = zz.sqrt().floor();
    let sqz = sqrtz * sqrtz;

    let mut r1: isize = 0;
    let mut r2: isize = 0;

    if (zz - sqz) >= sqrtz {
        r1 = sqrtz as isize;
        r2 = (zz - sqz - sqrtz) as isize;
    } else {
        r1 = (zz - sqz) as isize;
        r2 = sqrtz as isize;
    }

    if (r1 % 2) == 0 {
        (r1 / 2, r2 / 2)
    } else {
        ((r1 + 1) / -2, (r2 + 1) / -2)
    }
}

// ref:1 https://www.vertexfragment.com/ramblings/cantor-szudzik-pairing-functions/
