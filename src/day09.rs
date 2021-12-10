use std::collections::{BinaryHeap, HashMap};

const INPUT: &'static str = include_str!("day09.txt");

const DELTA: [(i64, i64); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

struct Grid {
    inner: Vec<Vec<i64>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        Grid {
            inner: input
                .lines()
                .map(|s| {
                    s.chars()
                        .map(|c| c.to_string().parse::<i64>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

impl Grid {
    fn at(&self, x: i64, y: i64) -> Option<i64> {
        if x < 0
            || x >= self.inner.len() as i64
            || y < 0
            || y >= self.inner[x as usize].len() as i64
        {
            None
        } else {
            Some(self.inner[x as usize][y as usize])
        }
    }

    fn find_low(&self, x: i64, y: i64) -> Option<(i64, i64)> {
        if self.is_low(x, y) {
            return Some((x, y));
        }
        if let Some(p) = self.at(x, y) {
            if p == 9 {
                return None;
            }
            for d in DELTA {
                let (dx, dy) = (x + d.0, y + d.1);

                if self.at(dx, dy).filter(|&p1| p1 < p).is_some() {
                    return self.find_low(dx, dy);
                }
            }
        }
        None
    }

    fn is_low(&self, x: i64, y: i64) -> bool {
        for d in DELTA {
            let (dx, dy) = (x + d.0, y + d.1);

            if self
                .at(dx, dy)
                .zip(self.at(x, y))
                .filter(|(p1, p2)| p1 <= p2)
                .is_some()
            {
                return false;
            }
        }
        true
    }

    fn indices<'a>(&'a self) -> impl Iterator<Item = (i64, i64)> + 'a {
        (0..self.inner.len() as i64)
            .flat_map(|x| (0..self.inner[x as usize].len() as i64).map(move |y| (x, y)))
    }
}

fn problem1() -> i64 {
    let grid = Grid::from(INPUT);
    grid.indices()
        .filter(|(x, y)| grid.is_low(*x, *y))
        .filter_map(|(x, y)| grid.at(x, y).map(|p| p + 1))
        .sum()
}

fn problem2() -> i64 {
    let grid = Grid::from(INPUT);

    let mut lows: HashMap<_, _> = grid
        .indices()
        .filter(|(x, y)| grid.is_low(*x, *y))
        .map(|xy| (xy, 0i64))
        .collect();

    grid.indices()
        .filter_map(|(x, y)| grid.find_low(x, y))
        .for_each(|xy| *lows.get_mut(&xy).unwrap() += 1);

    lows.values()
        .collect::<BinaryHeap<_>>()
        .iter()
        .take(3)
        .fold(1, |a, &&b| a * b)
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
