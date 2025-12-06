use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let mut lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.trim()
                .split(" ")
                .filter(|c| c.len() > 0)
                .map(|s| s.to_string())
                .collect()
        })
        .collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut lens: Vec<usize> = Vec::with_capacity(cols);

    for i in 0..cols {
        lens.push(0);
    }

    for i in 0..(rows - 1) {
        for j in 0..cols {
            lens[j] = lines[i][j].len().max(lens[j]);
        }
    }

    for i in 0..(rows - 1) {
        for j in 0..cols {
            let s = lines[i][j].clone();
            let l = lens[j];
            if j % 2 == 0 {
                lines[i][j] = format!("{:<l$}", s);
            } else {
                lines[i][j] = format!("{:>l$}", s);
            }
        }
    }

    let mut acc = 0;
    for i in 0..cols {
        let mut accp = 0;
        let mut accm = 1;
        let op = &lines[(rows - 1)][i];
        let mut a: [String; 4] = [
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ];
        for j in 0..(rows - 1) {
            for k in 0..(lens[i]) {
                let c = &lines[j][i][k..(k + 1)];
                let s = a[k].to_owned() + if c != " " { c } else { "" };
                a[k] = s;
            }
        }
        println!("a {a:#?}");
        for j in a.iter() {
            if j.len() == 0 {
                continue;
            }
            let parsed = <usize>::from_str(j.as_str()).unwrap();
            match op.as_str() {
                "+" => accp = accp + parsed,
                "*" => accm = accm * parsed,
                _ => todo! {},
            };
        }
        println!("op {op} accp {accp} accm {accm}");
        match op.as_str() {
            "+" => acc = acc + accp,
            "*" => acc = acc + accm,
            _ => todo! {},
        }
    }
    println!("acc {acc}");
}
