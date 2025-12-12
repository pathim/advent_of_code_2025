use std::collections::HashMap;

use crate::{AocInput, AocResult};

fn count_paths(
    start: &str,
    conns: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(cached) = cache.get(start) {
        return *cached;
    }
    if let Some(s) = conns.get(start) {
        let res = s.iter().map(|c| count_paths(c, conns, cache)).sum::<u64>();
        cache.insert(start.to_string(), res);
        res
    } else {
        0
    }
}

pub fn f(input: AocInput) -> AocResult {
    let mut conns = HashMap::new();
    for line in input.lines().flatten() {
        let (src, dests) = line.split_once(':').unwrap();
        let dests = dests
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        conns.insert(src.to_string(), dests);
    }
    let conns = conns;
    let mut cache = HashMap::new();
    cache.insert("out".to_string(), 1);
    let res1 = count_paths("you", &conns, &mut cache);

    // relies on there being no paths from dac to fft
    let mut fft_cache = HashMap::new();
    fft_cache.insert("fft".to_string(), 1);
    let svr_fft = count_paths("svr", &conns, &mut fft_cache);

    let mut dac_cache = HashMap::new();
    dac_cache.insert("dac".to_string(), 1);
    let fft_dac = count_paths("fft", &conns, &mut dac_cache);

    let dac_out = count_paths("dac", &conns, &mut cache);

    let res2 = svr_fft * fft_dac * dac_out;
    (res1, res2).into()
}
