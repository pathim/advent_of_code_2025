use crate::{AocInput, AocResult};

pub fn f(input: AocInput) -> AocResult {
    let mut res1 = 0;
    for line in input.lines().flatten() {
        if !line.contains('x') {
            continue;
        }
        let (size, amount) = line.split_once(':').unwrap();
        let (width, height) = size.split_once('x').unwrap();
        let width = width.parse::<u64>().unwrap();
        let height = height.parse::<u64>().unwrap();
        let mut amounts = amount
            .trim()
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let num02 = amounts[0].min(amounts[2]);
        amounts[0] -= num02;
        amounts[2] -= num02;
        let num13 = amounts[1].min(amounts[3]);
        amounts[1] -= num13;
        amounts[3] -= num13;
        let size = amounts.iter().sum::<u64>() * 9 + 12 * num02 + 15 * num13;
        if size < width * height {
            res1 += 1;
        }
    }
    res1.into()
}
