use crate::AocInput;

pub fn f(input: AocInput) -> crate::AocResult {
    let mut res1 = 0;
    for line in input.lines() {
        let line = line
            .unwrap()
            .chars()
            .map(|c| (c as u64) - ('0' as u64))
            .collect::<Vec<_>>();
        let max1 = line.iter().take(line.len() - 1).copied().enumerate().fold(
            (0usize, 0u64),
            |oldmax, current| {
                if current.1 > oldmax.1 {
                    current
                } else {
                    oldmax
                }
            },
        );
        let max2 = *line.iter().skip(max1.0 + 1).max().unwrap();
        res1 += max1.1 * 10 + max2;
    }
    res1.into()
}
