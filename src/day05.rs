use std::collections::HashMap;

const INPUT: &'static str = include_str!("day05.txt");

struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl From<&str> for Line {
    fn from(line: &str) -> Self {
        let (p1, p2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(",").unwrap();
        let (x2, y2) = p2.split_once(",").unwrap();
        Line {
            x1: x1.parse().unwrap(),
            y1: y1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y2: y2.parse().unwrap(),
        }
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_vertical(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_diagonal(&self) -> bool {
        (self.y1 - self.y2).abs() == (self.x1 - self.x2).abs()
    }
}

fn problem1() -> usize {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new(); 
    INPUT
        .lines()
        .map(Line::from)
        .for_each(|l| {
            if l.is_horizontal() || l.is_vertical() {
                let xs = (l.x2 - l.x1).signum();
                let ys = (l.y2 - l.y1).signum();
                let (mut x, mut y) = (l.x1, l.y1);
                while (x, y) != (l.x2 + xs, l.y2 + ys) {
                    *map.entry((x, y)).or_insert(0) += 1;
                    x += xs;
                    y += ys;
                }
            }
        });
    map.values().filter(|&&s| s > 1).count()
}

fn problem2() -> usize {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new(); 
    INPUT
        .lines()
        .map(Line::from)
        .for_each(|l| {
            if l.is_horizontal() || l.is_vertical() || l.is_diagonal() {
                let xs = (l.x2 - l.x1).signum();
                let ys = (l.y2 - l.y1).signum();
                let (mut x, mut y) = (l.x1, l.y1);
                while (x, y) != (l.x2 + xs, l.y2 + ys) {
                    *map.entry((x, y)).or_insert(0) += 1;
                    x += xs;
                    y += ys;
                }
            }
        });
    map.values().filter(|&&s| s > 1).count()
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
