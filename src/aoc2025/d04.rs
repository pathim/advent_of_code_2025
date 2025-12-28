use crate::{AocInput, AocResult, Grid, grid::V2d};

fn find_moveable(grid: &Grid) -> Vec<V2d> {
    grid.locations[&'@']
        .iter()
        .filter(|roll| {
            roll.neighbors8()
                .iter()
                .filter(|n| grid.is_char(**n, '@'))
                .count()
                < 4
        })
        .copied()
        .collect()
}

pub fn f(input: AocInput) -> AocResult {
    let mut res2 = 0;
    let mut grid = crate::Grid::new(input, &['@']);
    let mut to_remove = find_moveable(&grid);
    let res1 = to_remove.len();

    while !to_remove.is_empty() {
        res2 += to_remove.len();
        for p in to_remove.drain(..) {
            if let Some(v) = grid.get_mut(p) {
                *v = '.';
            }
            grid.locations.get_mut(&'@').unwrap().remove(&p);
        }
        to_remove = find_moveable(&grid);
    }

    (res1, res2).into()
}
