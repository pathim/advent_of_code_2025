use crate::AocInput;

pub fn f(input: AocInput) -> crate::AocResult {
    let mut pos = 50;
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (dir, dist) = line.split_at(1);
        let mut dist = dist.parse::<i32>().unwrap();

        res2 += dist / 100;
        dist %= 100;
        let lastpos = pos;
        pos += if dir == "L" { -dist } else { dist };
        let oldpos = pos;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            res1 += 1;
        }
        if (pos != oldpos || pos == 0) && lastpos != 0 {
            res2 += 1;
        }
    }
    (res1, res2).into()
}
