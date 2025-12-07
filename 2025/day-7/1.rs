use std::fs;
use std::str::FromStr;

fn main() {
    p1();
}

fn p1() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().split("").collect())
        .collect();
    dfs(lines.clone());
}

fn dfs(mut lines: Vec<Vec<&str>>) {
    let s_row = 0;
    let s_col = 8;
    let mut split = 0;
    visit(s_row + 1, s_col, &mut split, &mut lines);
    println!("{}", split);
}

fn visit(row: usize, col: usize, s: &mut usize, lines: &mut Vec<Vec<&str>>) {
    if row == (lines.len() - 1) {
        return;
    }
    let pos_char = lines[row][col];
    let adj = if pos_char == "." {
        lines[row][col] = "|";
        visit(row + 1, col, s, lines);
    } else if pos_char == "^" {
        *s += 1;
        visit(row, col - 1, s, lines);
        visit(row, col + 1, s, lines)
    };
}
