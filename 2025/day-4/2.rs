use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let input = fs::read_to_string("input2.txt").unwrap();

    let mut wall: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let rows = wall.len();
    let cols = wall[0].len();
    let mut acc = 0;

    let mut changed = true;
    while changed {
        changed = false;
        for r in 0..rows {
            for c in 0..cols {
                if wall[r][c] != '@' {
                    continue;
                }
                if should_replace(r, c, &wall) {
                    wall[r][c] = 'x';
                    acc += 1;
                    changed = true;
                }
            }
        }
    }
    println!("{:#?}", acc);
}

fn should_replace(r: usize, c: usize, wall: &[Vec<char>]) -> bool {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let rows = wall.len() as isize;
    let cols = wall[0].len() as isize;
    let mut neighbors = 0;

    for (dr, dc) in directions {
        let nr = dr + (r as isize);
        let nc = dc + (c as isize);

        if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
            if wall[nr as usize][nc as usize] == '@' {
                neighbors += 1;
            }
        }

        if neighbors >= 4 {
            return false;
        }
    }

    true
}
