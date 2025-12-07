use std::collections::{HashMap, HashSet};

use crate::{grid::V2d, AocInput, AocResult, Grid};

fn count_timelines(ray: V2d, grid: &Grid, cache: &mut HashMap<V2d, u64>) -> u64 {
    if let Some(res) = cache.get(&ray) {
        return *res;
    }
    let d = V2d(1, 0);
    for i in ray.1 + 1..grid.size.1 {
        let p = V2d(ray.0, i);
        if grid.get(p) == Some('^') {
            let res = count_timelines(p + d, grid, cache) + count_timelines(p - d, grid, cache);
            cache.insert(ray, res);
            return res;
        }
    }
    1
}

pub fn f(input: AocInput) -> AocResult {
    let mut grid = Grid::new(input, &['S', '^']);
    let mut rays = Vec::new();
    rays.push(*grid.locations[&'S'].iter().next().unwrap());
    let mut cache = HashMap::new();
    let res2 = count_timelines(*rays.first().unwrap(), &grid, &mut cache);
    let mut res1 = 0;
    let d = V2d(1, 0);
    grid.overlay.insert('X', HashSet::new());
    while let Some(ray) = rays.pop() {
        for i in ray.1 + 1..grid.size.1 {
            let p = V2d(ray.0, i);
            if grid.get(p) == Some('^') {
                if grid.overlay.get_mut(&'X').unwrap().insert(p) {
                    res1 += 1;
                    for new_ray in [p + d, p - d] {
                        let new_pos = grid.get_mut(new_ray).unwrap();
                        if *new_pos == '.' {
                            rays.push(new_ray);
                            *new_pos = '|';
                        }
                    }
                }
                break;
            }
        }
    }
    (res1, res2).into()
}
