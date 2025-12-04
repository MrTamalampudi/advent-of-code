use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let is = fs::read_to_string("input2.txt").unwrap();
    let banks: Vec<_> = is.trim().split("\n").collect();
    println!("banks {banks:#?}");
    let mut acc = 0;
    for bank in banks.iter() {
        let mut max: usize = 0;
        for j in 0..bank.len() - 1 {
            let a = &bank[j..j + 1];
            for k in j + 1..bank.len() {
                let b = &bank[k..k + 1];
                let num_s = format!("{a}{b}");
                let num = <usize>::from_str(&num_s[..]).unwrap();

                if num > max {
                    max = num;
                }
            }
        }
        acc += max
    }
    println!("{acc:#?}");
}
