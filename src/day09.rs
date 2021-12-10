use crate::utils::*;
use std::collections::{BTreeSet, HashMap};

const INPUT: &'static str = include_str!("day09.txt");

struct Map {
    grid: Grid<i64>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        Map {
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

impl Map {
    fn find_low(&self, idx: GridIndex) -> Option<GridIndex> {
        if self.is_low(idx) {
            return Some(idx);
        }
        if let Some(p) = self.grid.at(idx) {
            if p == &9 {
                return None;
            }
            return idx
                .nswe_neighbors()
                .filter_map(|nidx| {
                    self.grid
                        .at(nidx)
                        .filter(|&p1| p1 < p)
                        .map(|_| self.find_low(nidx))
                })
                .next()
                .flatten();
        }
        None
    }

    fn is_low(&self, idx: GridIndex) -> bool {
        idx.nswe_neighbors().all(|nidx| {
            self.grid
                .at(nidx)
                .zip(self.grid.at(idx))
                .filter(|(p1, p2)| p1 <= p2)
                .is_none()
        })
    }
}

fn problem1() -> i64 {
    let map = Map::from(INPUT);
    map.grid
        .indices()
        .filter(|idx| map.is_low(*idx))
        .filter_map(|idx| map.grid.at(idx).map(|p| p + 1))
        .sum()
}

fn problem2() -> i64 {
    let map = Map::from(INPUT);

    let mut lows: HashMap<_, _> = map
        .grid
        .indices()
        .filter(|idx| map.is_low(*idx))
        .map(|idx| (idx, 0i64))
        .collect();

    map.grid
        .indices()
        .filter_map(|idx| map.find_low(idx))
        .for_each(|xy| *lows.get_mut(&xy).unwrap() += 1);

    lows.values()
        .map(|i| -i)
        .collect::<BTreeSet<_>>()
        .iter()
        .map(|i| -i)
        .take(3)
        .fold(1, |a, b| a * b)
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
