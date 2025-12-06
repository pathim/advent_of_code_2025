use std::ops::{Add, Mul};

use crate::{AocInput, AocResult};

enum Operations {
    Add,
    Mul,
}

impl Operations {
    fn exec<T: Add<Output = T> + Mul<Output = T>>(&self, a: T, b: T) -> T {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
    fn init(&self) -> u64 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }
}

pub fn f(input: AocInput) -> AocResult {
    let mut numbers = Vec::new();
    let mut lines = Vec::new();
    let mut operations = Vec::new();
    for l in input.lines().flatten() {
        let mut current_line = Vec::new();
        let mut current_number = None;
        lines.push(l.chars().collect::<Vec<_>>());
        for c in l.chars() {
            if c.is_whitespace() {
                if let Some(v) = current_number.take() {
                    current_line.push(v);
                }
                continue;
            }
            if c == '+' {
                operations.push(Operations::Add);
                continue;
            }
            if c == '*' {
                operations.push(Operations::Mul);
                continue;
            }
            let num = c as u64 - '0' as u64;
            let cur = current_number.get_or_insert(0);
            *cur *= 10;
            *cur += num;
        }
        if let Some(v) = current_number {
            current_line.push(v);
        }
        if !current_line.is_empty() {
            numbers.push(current_line);
        }
    }
    let mut res1 = 0;
    for (i, o) in operations.iter().enumerate() {
        res1 += numbers
            .iter()
            .map(|v| v.get(i).unwrap())
            .copied()
            .fold(o.init(), |a, b| o.exec(a, b));
    }
    lines.pop();
    let mut o_idx = 0;
    let mut op = &operations[o_idx];
    let mut val = op.init();
    let mut res2 = 0;
    for col in 0..lines[0].len() {
        let num = lines
            .iter()
            .map(|l| l[col])
            .filter(|c| !c.is_whitespace())
            .fold(0, |a, b| a * 10 + (b as u64) - ('0' as u64));
        if num == 0 {
            res2 += val;
            o_idx += 1;
            op = &operations[o_idx];
            val = op.init();
        } else {
            val = op.exec(val, num);
        }
    }
    res2 += val;
    (res1, res2).into()
}
