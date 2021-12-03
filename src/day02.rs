const INPUT: &'static str = include_str!("day02.txt");

enum Direction {
    Forward,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => unreachable!(),
        }
    }
}

struct Instruction {
    dir: Direction,
    value: i64,
}

impl From<(&str, &str)> for Instruction {
    fn from((dir, n): (&str, &str)) -> Self {
        Instruction {
            dir: dir.into(),
            value: n.parse::<i64>().unwrap(),
        }
    }
}

struct Sub1 {
    depth: i64,
    horiz: i64,
}

impl Sub1 {
    fn new() -> Self {
        Self { depth: 0, horiz: 0 }
    }

    fn apply(self, instr: Instruction) -> Self {
        match instr.dir {
            Direction::Forward => Self {
                depth: self.depth,
                horiz: self.horiz + instr.value,
            },
            Direction::Down => Self {
                depth: self.depth + instr.value,
                horiz: self.horiz,
            },
            Direction::Up => Self {
                depth: self.depth - instr.value,
                horiz: self.horiz,
            },
        }
    }

    fn mul(self) -> i64 {
        self.depth * self.horiz
    }
}

struct Sub2 {
    depth: i64,
    horiz: i64,
    aim: i64,
}

impl Sub2 {
    fn new() -> Self {
        Self {
            depth: 0,
            horiz: 0,
            aim: 0,
        }
    }

    fn apply(self, instr: Instruction) -> Self {
        match instr.dir {
            Direction::Forward => Self {
                horiz: self.horiz + instr.value,
                depth: self.depth + self.aim * instr.value,
                ..self
            },
            Direction::Down => Self {
                aim: self.aim + instr.value,
                ..self
            },
            Direction::Up => Self {
                aim: self.aim - instr.value,
                ..self
            },
        }
    }

    fn mul(self) -> i64 {
        self.depth * self.horiz
    }
}

fn problem1() -> i64 {
    INPUT
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(Instruction::from)
        .fold(Sub1::new(), |state, instr| state.apply(instr))
        .mul()
}

fn problem2() -> i64 {
    INPUT
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(Instruction::from)
        .fold(Sub2::new(), |state, instr| state.apply(instr))
        .mul()
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
