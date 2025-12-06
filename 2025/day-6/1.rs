use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().split(" ").filter(|c| c.len() > 0).collect())
        .collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut acc = 0;
    for i in 0..cols {
        let mut accp = 0;
        let mut accm = 1;
        let op = lines[(rows - 1)][i];
        for j in 0..(rows - 1) {
            let parsed = <usize>::from_str(lines[j][i]).unwrap();
            match op {
                "+" => accp += parsed,
                "*" => accm = accm * parsed,
                _ => todo! {},
            };
        }
        match op {
            "+" => acc += accp,
            "*" => acc = acc + accm,
            _ => todo! {},
        }
    }
    println!("acc {acc}");
    // println!("{:#?}", lines);
}
