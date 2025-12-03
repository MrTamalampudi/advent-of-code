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
            for j in 1..=(len / 2) {
                if len % j == 0 {
                    let a = &its[0..j];
                    let mut valid_j = true;
                    for k in (j..len).step_by(j) {
                        let b = &its[k..(k + j)];
                        if a != b {
                            valid_j = false;
                            break;
                        }
                    }
                    if valid_j {
                        acc += i;
                        break;
                    }
                }
            }
        }
    }
    println!("{acc}");
}
