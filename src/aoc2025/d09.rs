use std::str::FromStr;

use crate::{AocInput, AocResult, grid::V2d};

enum Dir {
    H,
    V,
}
struct Line {
    c: isize,
    min: isize,
    max: isize,
    dir: Dir,
}

impl Line {
    fn new(a: &V2d, b: &V2d) -> Self {
        if a.0 == b.0 {
            Self {
                c: a.0,
                min: a.1.min(b.1),
                max: a.1.max(b.1),
                dir: Dir::V,
            }
        } else if a.1 == b.1 {
            Self {
                c: a.1,
                min: a.0.min(b.0),
                max: a.0.max(b.0),
                dir: Dir::H,
            }
        } else {
            panic!("invalid nodes {:?} {:?}", a, b);
        }
    }

    fn check_rect_collides(&self, min_node: &V2d, max_node: &V2d) -> bool {
        let (is_outside, min, max) = match self.dir {
            Dir::H => (
                self.c < min_node.1 || self.c > max_node.1,
                min_node.0,
                max_node.0,
            ),
            Dir::V => (
                self.c < min_node.0 || self.c > max_node.0,
                min_node.1,
                max_node.1,
            ),
        };
        if is_outside {
            return false;
        }
        if self.min > max || self.max < min {
            return false;
        }
        true
    }
}

fn get_inner_rect(c1: &V2d, c2: &V2d) -> (V2d, V2d) {
    let top_left = c1.min(c2) + V2d(1, 1);
    let bottom_right = c1.max(c2) - V2d(1, 1);
    (top_left, bottom_right)
}

pub fn f(input: AocInput) -> AocResult {
    let tiles = input
        .lines()
        .flatten()
        .map(|l| V2d::from_str(&l).unwrap())
        .collect::<Vec<_>>();
    let boundaries = tiles
        .windows(2)
        .chain(Some(
            [*tiles.last().unwrap(), *tiles.first().unwrap()].as_slice(),
        ))
        .map(|w| Line::new(&w[0], &w[1]))
        .collect::<Vec<_>>();
    let mut res1 = 0;
    let mut res2 = 0;
    for (i, n) in tiles.iter().enumerate() {
        for n2 in tiles.iter().skip(i) {
            let (r_min, r_max) = get_inner_rect(n, n2);
            let collides = boundaries
                .iter()
                .any(|b| b.check_rect_collides(&r_min, &r_max));
            let a = (n - n2).abs();
            let area = (a.0 + 1) * (a.1 + 1);
            res1 = res1.max(area);
            if !collides {
                res2 = res2.max(area);
            }
        }
    }
    (res1, res2).into()
}
