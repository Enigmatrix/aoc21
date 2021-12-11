use std::collections::{HashSet, VecDeque};

use crate::utils::Grid;

const INPUT: &'static str = include_str!("day11.txt");

struct Jellyfishes {
    grid: Grid<i64>,
}

impl From<&str> for Jellyfishes {
    fn from(input: &str) -> Self {
        Jellyfishes {
            grid: Grid::new(
                input
                    .lines()
                    .map(|s| {
                        s.chars()
                            .map(|c| c.to_string().parse::<i64>().unwrap())
                            .collect()
                    })
                    .collect(),
            ),
        }
    }
}

fn problem1() -> i64 {
    let mut map = Jellyfishes::from(INPUT);
    let mut ans = 0;

    for _ in 0..100 {
        let mut cur_flashes = VecDeque::new();
        let mut flashed = HashSet::new();

        for idx in map.grid.indices().collect::<Vec<_>>() {
            map.grid[idx] += 1;
        }

        map.grid
            .indices()
            .filter(|idx| map.grid[*idx] > 9)
            .for_each(|idx| {
                flashed.insert(idx);
                cur_flashes.push_back(idx);
            });

        while !cur_flashes.is_empty() {
            for idx in cur_flashes
                .iter()
                .flat_map(|idx| map.grid.all_neighbors(*idx))
                .collect::<Vec<_>>()
            {
                map.grid[idx] += 1;
            }
            cur_flashes.clear();

            map.grid
                .indices()
                .filter(|idx| map.grid[*idx] > 9)
                .for_each(|idx| {
                    if flashed.contains(&idx) {
                        return;
                    }
                    flashed.insert(idx);
                    cur_flashes.push_back(idx);
                });
        }

        flashed.iter().for_each(|idx| map.grid[*idx] = 0);

        ans += flashed.len() as i64;
    }
    ans
}

fn problem2() -> i64 {
    let mut map = Jellyfishes::from(INPUT);
    let mut step = 0;

    loop {
        let mut cur_flashes = VecDeque::new();
        let mut flashed = HashSet::new();

        for idx in map.grid.indices().collect::<Vec<_>>() {
            map.grid[idx] += 1;
        }

        map.grid
            .indices()
            .filter(|idx| map.grid[*idx] > 9)
            .for_each(|idx| {
                flashed.insert(idx);
                cur_flashes.push_back(idx);
            });

        while !cur_flashes.is_empty() {
            for idx in cur_flashes
                .iter()
                .flat_map(|idx| map.grid.all_neighbors(*idx))
                .collect::<Vec<_>>()
            {
                map.grid[idx] += 1;
            }
            cur_flashes.clear();

            map.grid
                .indices()
                .filter(|idx| map.grid[*idx] > 9)
                .for_each(|idx| {
                    if flashed.contains(&idx) {
                        return;
                    }
                    flashed.insert(idx);
                    cur_flashes.push_back(idx);
                });
        }

        flashed.iter().for_each(|idx| map.grid[*idx] = 0);
        step += 1;

        if map.grid.indices().all(|idx| map.grid[idx] == 0) {
            break step;
        }
    }
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
