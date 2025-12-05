use std::collections::HashSet;
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
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut result: Vec<Vec<usize>> = vec![];
    result.push(ranges[0].clone());
    for range in ranges.iter().skip(1) {
        let last = result.last_mut().unwrap();
        if range[0] <= last[1] {
            last[1] = last[1].max(range[1]);
        } else {
            result.push(range.clone());
        }
    }
    let mut acc = 0;
    for r in result.iter() {
        acc += (r[0]..(r[1] + 1)).len();
    }
    println!("{:#?}", acc);
}
