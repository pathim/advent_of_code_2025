use crate::{AocInput, AocResult};

fn led_to_num(leds: &str) -> u64 {
    leds.chars()
        .rev()
        .fold(0, |acc, v| acc * 2 + (if v == '#' { 1 } else { 0 }))
}

fn get_switch_masks(switches: &str) -> Vec<u64> {
    let mut res = Vec::new();
    for s in switches.split(' ') {
        let a = s.split_at(1).1.split_at(s.len() - 2).0;
        res.push(
            a.split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .fold(0, |acc, v| acc + (1 << v)),
        );
    }
    res
}

fn find_switches(num: u64, sw: &Vec<u64>) -> u64 {
    let mut res = u64::MAX;
    for n in 1u16..=(1 << sw.len()) - 1 { // optimization: better order of iteration + early return
        let sw_res = sw.iter().enumerate().fold(0, |acc, (i, v)| {
            acc ^ (if (1 << i) & n != 0 { *v } else { 0 })
        });
        if sw_res == num {
            res = res.min(n.count_ones() as u64);
        }
    }
    res
}

pub fn f(input: AocInput) -> AocResult {
    let mut res1 = 0;
    for l in input.lines().flatten() {
        let (leds, rest) = l.split_once(']').unwrap();
        let (_, leds) = leds.split_at(1);
        let (switches, jolt) = rest.split_once('{').unwrap();
        let switches = switches.trim();
        let (jolt, _) = jolt.split_once('}').unwrap();
        let num = led_to_num(leds);
        let sw = get_switch_masks(switches);
        let min_switches = find_switches(num, &sw);
        res1 += min_switches;
    }
    res1.into()
}
