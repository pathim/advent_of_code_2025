use std::str::FromStr;

use crate::{grid::V2d, AocInput, AocResult};

pub fn f(input: AocInput) -> AocResult {
    let tiles = input
        .lines()
        .flatten()
        .map(|l| V2d::from_str(&l).unwrap())
        .collect::<Vec<_>>();
    let mut res1 = 0;
    for (i, n) in tiles.iter().enumerate() {
        for n2 in tiles.iter().skip(i) {
            let a = n - n2;
            let area = (a.0 + 1) * (a.1 + 1);
            res1 = res1.max(area);
        }
    }
    res1.into()
}
