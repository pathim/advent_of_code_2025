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

fn solve_jolts(switches: &str, jolt: &str) -> u64 {
    let mut switch_maps = Vec::new();
    for s in switches.split(' ') {
        let a = s.split_at(1).1.split_at(s.len() - 2).0;
        switch_maps.push(
            a.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    let max_led = switch_maps.iter().flatten().max().unwrap();
    let num_switches = switch_maps.len();
    let sw_vars = (0..num_switches)
        .map(|_| z3::ast::Int::fresh_const("sw"))
        .collect::<Vec<_>>();
    let mut sums = (0..=*max_led)
        .map(|_| z3::ast::Int::from_u64(0))
        .collect::<Vec<_>>();
    for (leds, sw) in switch_maps.iter().zip(sw_vars.iter()) {
        for l in leds {
            sums[*l] += sw;
        }
    }
    let vals = jolt
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let solver = z3::Solver::new();
    for (sum, joltage) in sums.iter().zip(vals.iter()) {
        solver.assert(sum.eq(*joltage));
    }
    for sw in sw_vars.iter() {
        solver.assert(sw.ge(0));
    }
    let mut res = u64::MAX;
    for sol in solver.solutions(sw_vars, true) {
        let presses = sol.iter().map(|x| x.as_u64().unwrap()).sum::<u64>();
        res = res.min(presses);
    }
    res
}

fn find_switches(num: u64, sw: &[u64]) -> u64 {
    let mut res = u64::MAX;
    for n in 1u16..=(1 << sw.len()) - 1 {
        // optimization: better order of iteration + early return
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
    let mut res2 = 0;
    for l in input.lines().flatten() {
        let (leds, rest) = l.split_once(']').unwrap();
        let (_, leds) = leds.split_at(1);
        let (switches, jolt) = rest.split_once('{').unwrap();
        let switches = switches.trim();
        let (jolt, _) = jolt.split_once('}').unwrap();
        let num = led_to_num(leds);
        //let buttons_to_press=sw_pinv*led_vec;
        let sw = get_switch_masks(switches);
        let min_switches = find_switches(num, &sw);
        res2 += solve_jolts(switches, jolt);
        res1 += min_switches;
    }
    (res1, res2).into()
}
// 16326 too low
// 16327 too low
// 16782 too high
