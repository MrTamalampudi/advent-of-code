use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let is = fs::read_to_string("input2.txt").unwrap();
    let ranges: Vec<_> = is
        .trim()
        .split(",")
        .map(|i| {
            let a: Vec<_> = i.split("-").collect();
            let start = <usize>::from_str(a[0]).unwrap();
            let end = <usize>::from_str(a[1]).unwrap();
            (start, end)
        })
        .collect();
    let mut acc = 0;
    for (s, e) in ranges.iter() {
        for i in *s..=*e {
            let its = i.to_string();
            let len = its.len();
            if len % 2 == 1 {
                continue;
            }
            let fp = &its[..(len / 2)];
            let sp = &its[(len / 2)..];
            if fp == sp {
                acc += i;
            }
        }
    }

    println!("{acc:#?}");
}
