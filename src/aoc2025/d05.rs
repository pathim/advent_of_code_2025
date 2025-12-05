use std::ops::RangeInclusive;

pub fn union<T: Ord + Clone>(
    r1: &RangeInclusive<T>,
    r2: &RangeInclusive<T>,
) -> Option<RangeInclusive<T>> {
    let start = r1.start().min(r2.start()).clone();
    let end = r1.end().max(r2.end()).clone();
    let res = start..=end;
    (r2.contains(r1.start()) || r2.contains(r1.end())).then_some(res)
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut lines = input.lines().flatten();
    let mut ranges = Vec::new();
    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }
        let (start, end) = l.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        ranges.push(start..=end);
    }
    'outer: for l in lines {
        let val = l.parse::<u64>().unwrap();
        for r in ranges.iter() {
            if r.contains(&val) {
                res1 += 1;
                continue 'outer;
            }
        }
    }
    ranges.sort_by_key(|r| *r.start());
    let mut non_overlapping_ranges = Vec::new();
    non_overlapping_ranges.push(ranges[0].clone());
    for r in ranges {
        let r2 = non_overlapping_ranges.pop().unwrap();
        if let Some(ur) = union(&r, &r2) {
            non_overlapping_ranges.push(ur);
        } else {
            non_overlapping_ranges.push(r2);
            non_overlapping_ranges.push(r);
        }
    }
    let res2 = non_overlapping_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<u64>();

    (res1, res2).into()
}
