use std::fs;
use std::str::FromStr;

fn main() {
    p2();
}

fn p2() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let input: Vec<_> = input_string.trim().split("\n").collect();
    let mut arrow = 50;
    let mut z_c = 0;
    for i in input.iter() {
        let shift = &i[..1];
        let num = <isize>::from_str(&i[1..]).unwrap();
        let a;
        match shift {
            "L" => {
                a = arrow - num;
            }
            "R" => {
                a = arrow + num;
            }
            _ => todo!(),
        }
        println!("{shift} {num} {arrow} {a}");
        let z_t = a / 100;
        if a <= 0 && arrow != 0 {
            z_c = z_c + z_t.abs() + 1;
        } else if a >= 100 {
            z_c = z_c + z_t.abs();
        } else if a <= 0 && arrow == 0 {
            z_c = z_c + z_t.abs();
        }
        arrow = a.rem_euclid(100);
    }

    println!("{z_c}");
}
