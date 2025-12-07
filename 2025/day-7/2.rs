use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().split("").collect())
        .collect();
    dfs(lines.clone());
}

fn dfs(mut lines: Vec<Vec<&str>>) {
    let s_row = 0;
    let s_col = 71;
    let mut split = 0;
    let mut mem: HashMap<(usize, usize), usize> = HashMap::new();
    let split = visit(s_row + 1, s_col, &mut split, &mut lines, &mut mem);
    println!("{}", split);
}

fn visit(
    row: usize,
    col: usize,
    s: &mut usize,
    lines: &mut Vec<Vec<&str>>,
    mem: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let entry = mem.get(&(row, col));
    if let Some(dist) = entry {
        return *dist;
    }
    if row == (lines.len()) {
        return 1;
    }
    let mut a = 0;
    let pos_char = lines[row][col];
    let adj = if pos_char == "." {
        a = visit(row + 1, col, s, lines, mem);
    } else if pos_char == "^" {
        a = visit(row, col - 1, s, lines, mem) + visit(row, col + 1, s, lines, mem);
    };

    mem.insert((row, col), a);
    return a;
}
