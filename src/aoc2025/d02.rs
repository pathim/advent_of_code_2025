use crate::AocInput;

fn is_invalid1(n: u64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();
    if !b.len().is_multiple_of(2) {
        return false;
    }
    let mid = b.len() / 2;
    b[..mid] == b[mid..]
}

fn is_invalid2(n: u64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();
    'outer: for l in 1..=b.len() / 2 {
        let chunk = b.chunks_exact(l);
        if !chunk.remainder().is_empty() {
            continue;
        }
        let mut val = None;
        for c in chunk {
            if let Some(v) = val {
                if v != c {
                    continue 'outer;
                }
            } else {
                val = Some(c);
            }
        }
        return true;
    }
    false
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    let line = input.lines().next().unwrap().unwrap();
    for r in line.split(',') {
        let (start, end) = r.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        for i in start..=end {
            if is_invalid1(i) {
                res1 += i;
            }
            if is_invalid2(i) {
                res2 += i;
            }
        }
    }
    (res1, res2).into()
}
