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

impl TryFrom<char> for Operations {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => Err(()),
        }
    }
}

pub fn f(input: AocInput) -> AocResult {
    let mut numbers = Vec::new();
    let mut numbers2 = Vec::new();
    let mut operations = Vec::new();
    for l in input.lines().flatten() {
        let mut current_line = Vec::new();
        let mut current_number = None;
        for (i, c) in l.chars().enumerate() {
            if numbers2.len() < i + 1 {
                numbers2.push(0u64);
            }
            let val = numbers2.get_mut(i).unwrap();
            let num = c.to_digit(10).map(|x| x as u64);
            if let Some(n) = num {
                *val = *val * 10 + n;
            }
            if c.is_whitespace() {
                if let Some(v) = current_number.take() {
                    current_line.push(v);
                }
                continue;
            }
            if let Ok(op) = Operations::try_from(c) {
                operations.push(op);
                continue;
            }
            let cur = current_number.get_or_insert(0);
            *cur *= 10;
            *cur += num.unwrap();
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
    let res2 = numbers2
        .split(|a| *a == 0)
        .zip(operations.iter())
        .map(|(ns, op)| ns.iter().fold(op.init(), |a, b| op.exec(a, *b)))
        .sum::<u64>();
    (res1, res2).into()
}
