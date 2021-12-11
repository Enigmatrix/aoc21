use std::ops::{Add, Index, IndexMut};

const NSWE_DELTA: [(i64, i64); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
const ALL_DELTA: [(i64, i64); 8] = [
    (0, 1),
    (1, 0),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridIndex {
    x: i64,
    y: i64,
}

impl GridIndex {
    pub fn nswe_neighbors(&self) -> impl Iterator<Item = GridIndex> + '_ {
        NSWE_DELTA.iter().map(|d| self.clone() + d)
    }

    pub fn all_neighbors(&self) -> impl Iterator<Item = GridIndex> + '_ {
        ALL_DELTA.iter().map(|d| self.clone() + d)
    }
}

impl Add<&(i64, i64)> for GridIndex {
    type Output = GridIndex;

    fn add(self, rhs: &(i64, i64)) -> Self::Output {
        GridIndex {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Index<GridIndex> for Grid<T> {
    type Output = T;

    fn index(&self, index: GridIndex) -> &Self::Output {
        &self.inner[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<GridIndex> for Grid<T> {
    fn index_mut(&mut self, index: GridIndex) -> &mut Self::Output {
        &mut self.inner[index.x as usize][index.y as usize]
    }
}

impl<T> Grid<T> {
    pub fn indices(&self) -> impl Iterator<Item = GridIndex> + '_ {
        (0..self.inner.len() as i64)
            .flat_map(|x| (0..self.inner[x as usize].len() as i64).map(move |y| GridIndex { x, y }))
    }

    pub fn new(inner: Vec<Vec<T>>) -> Self {
        Grid { inner }
    }

    pub fn nswe_neighbors(&self, idx: GridIndex) -> Vec<GridIndex> {
        idx.nswe_neighbors()
            .filter(|nidx| self.valid(*nidx))
            .collect()
    }

    pub fn all_neighbors(&self, idx: GridIndex) -> Vec<GridIndex> {
        idx.all_neighbors()
            .filter(|nidx| self.valid(*nidx))
            .collect()
    }

    pub fn valid(&self, idx: GridIndex) -> bool {
        !(idx.x < 0
            || idx.x >= self.inner.len() as i64
            || idx.y < 0
            || idx.y >= self.inner[idx.x as usize].len() as i64)
    }

    pub fn at(&self, idx: GridIndex) -> Option<&T> {
        if self.valid(idx) {
            None
        } else {
            Some(&self[idx])
        }
    }
}
