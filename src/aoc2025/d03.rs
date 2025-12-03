use crate::AocInput;

fn find_max_first(values: &[u64], start: usize, end: usize) -> (usize, u64) {
    let mut oldmax = (0, 0);
    for v in values[start..end].iter().copied().enumerate() {
        if v.1 > oldmax.1 {
            oldmax = v;
        }
    }
    oldmax
}

fn find_joltage(values: &[u64], n:usize) -> u64 {
    let mut start = 0;
    let mut result = 0;
    for i in 0..n {
        let max = find_max_first(values, start, values.len() - n + i + 1);
        result *= 10;
        result += max.1;
        start += max.0 + 1;
    }
    result
}

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let line = line
            .unwrap()
            .chars()
            .map(|c| (c as u64) - ('0' as u64))
            .collect::<Vec<_>>();

        res1 += find_joltage(&line, 2);
        let jolt = find_joltage(&line, 12);
        res2 += jolt;
    }
    (res1, res2).into()
}
