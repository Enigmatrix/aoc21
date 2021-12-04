const INPUT: &'static str = include_str!("day04.txt");

#[derive(Debug)]
struct Board {
    inner: Vec<Vec<(u64, bool)>>,
}

impl Board {
    fn read_board<'a>(str: impl Iterator<Item = &'a &'a str>) -> Board {
        Board {
            inner: str
                .map(|row| {
                    row.split_whitespace()
                        .map(|s| (s.parse().unwrap(), false))
                        .collect()
                })
                .collect(),
        }
    }

    fn read_boards<'a>(str: &[&'a str]) -> Vec<Board> {
        str.chunks_exact(6)
            .map(|strs| Board::read_board(strs.into_iter().take(5)))
            .collect()
    }

    fn mark(&mut self, n: u64) {
        for i in 0..self.inner.len() {
            for j in 0..self.inner[i].len() {
                if self.inner[i][j].0 == n {
                    self.inner[i][j].1 = true;
                    return;
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        let len = self.inner.len();

        for x in 0..len {
            let mut t = true;
            for y in 0..len {
                if !self.inner[x][y].1 {
                    t = false;
                    break;
                }
            }
            if t {
                return true;
            }
        }

        for x in 0..len {
            let mut t = true;
            for y in 0..len {
                if !self.inner[y][x].1 {
                    t = false;
                    break;
                }
            }
            if t {
                return true;
            }
        }
        return false;
    }

    fn incomplete_sum(&self) -> u64 {
        let len = self.inner.len();
        let mut sum = 0;

        for x in 0..len {
            for y in 0..len {
                if !self.inner[x][y].1 {
                    sum += self.inner[x][y].0;
                }
            }
        }

        return sum;
    }
}

fn problem1() -> u64 {
    let input: Vec<_> = INPUT.lines().collect();
    let numbers: Vec<u64> = input[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut boards = Board::read_boards(&input[2..]);

    for n in numbers {
        for b in boards.iter_mut() {
            b.mark(n);
            if b.is_complete() {
                return n * b.incomplete_sum();
            }
        }
    }
    0
}

fn problem2() -> u64 {
    let input: Vec<_> = INPUT.lines().collect();
    let numbers: Vec<u64> = input[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut boards = Board::read_boards(&input[2..]);

    let mut m = boards.len();

    for n in numbers {
        for b in boards.iter_mut() {
            if b.is_complete() {
                continue;
            }
            b.mark(n);
            if b.is_complete() {
                m -= 1;
                if m == 0 {
                    return n * b.incomplete_sum();
                }
            }
        }
    }
    0
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
