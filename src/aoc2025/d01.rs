use crate::AocInput;

pub fn f(input: AocInput) -> crate::AocResult {
    let mut pos = 50i32;
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (dir, dist) = line.split_at(1);
        let dist = dist.parse::<i32>().unwrap();
        for _ in 0..dist {
            pos += if dir == "L" { -1 } else { 1 };
            pos = pos.rem_euclid(100);
            if pos == 0 {
                res2 += 1;
            }
        }
        if pos == 0 {
            res1 += 1;
        }
    }
    (res1, res2).into()
}
