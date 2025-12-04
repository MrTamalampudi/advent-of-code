use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let is = fs::read_to_string("input2.txt").unwrap();
    let banks: Vec<_> = is.trim().split("\n").collect();
    let mut acc = 0;
    for bank in banks {
        let digits: Vec<u8> = bank
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect();

        let n = digits.len();
        let k = 12;

        if n < k {
            continue;
        }

        let mut to_drop = n - k;
        let mut stack: Vec<u8> = Vec::with_capacity(k);

        for val in digits {
            while to_drop > 0 && !stack.is_empty() && val > *stack.last().unwrap() {
                stack.pop();
                to_drop -= 1;
            }
            stack.push(val);
        }

        stack.truncate(k);

        let mut max_val: u64 = 0;
        for digit in stack {
            max_val = max_val * 10 + (digit as u64);
        }

        println!("max {max_val}");
        acc += max_val;
    }

    println!("{acc}");
}
