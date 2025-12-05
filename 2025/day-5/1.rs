use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let input = fs::read_to_string("input2.txt").unwrap();

    let mut ranges: Vec<_> = vec![];
    let mut available: Vec<usize> = vec![];
    let mut b_line = false;
    for line in input.lines() {
        if !b_line {
            if line == "" {
                b_line = true;
                continue;
            }
        }
        if !b_line {
            let a: Vec<_> = line.split("-").collect();
            let a: Vec<_> = a
                .iter()
                .map(|num| <usize>::from_str(num).unwrap())
                .collect();
            ranges.push(a);
        } else {
            let a = <usize>::from_str(line).unwrap();
            available.push(a);
        }
    }
    let mut acc = 0;
    for i in available.iter() {
        for range in ranges.iter() {
            if (range[0]..=range[1]).contains(i) {
                acc += 1;
                break;
            }
        }
    }
    println!("{acc}");
}
