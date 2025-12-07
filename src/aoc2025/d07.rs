use std::collections::{HashMap, HashSet};

use crate::{grid::V2d, AocInput, AocResult, Grid};

fn count_timelines(
    ray: V2d,
    grid: &Grid,
    cache: &mut HashMap<V2d, u64>,
    splitters: &mut HashSet<V2d>,
) -> u64 {
    if let Some(res) = cache.get(&ray) {
        return *res;
    }
    let d = V2d(1, 0);
    for i in ray.1 + 1..grid.size.1 {
        let p = V2d(ray.0, i);
        if grid.get(p) == Some('^') {
            splitters.insert(p);
            let res = count_timelines(p + d, grid, cache, splitters)
                + count_timelines(p - d, grid, cache, splitters);
            cache.insert(ray, res);
            return res;
        }
    }
    1
}

pub fn f(input: AocInput) -> AocResult {
    let grid = Grid::new(input, &['S']);
    let start = *grid.locations[&'S'].iter().next().unwrap();
    let mut cache = HashMap::new();
    let mut splitters = HashSet::new();
    let res2 = count_timelines(start, &grid, &mut cache, &mut splitters);
    let res1 = splitters.len();
    (res1, res2).into()
}
