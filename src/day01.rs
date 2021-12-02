struct Count {
    prev: u64,
    number: u64,
}

impl Count {
    pub fn new() -> Count {
        Count {
            prev: u64::MAX,
            number: 0,
        }
    }

    pub fn include(self, number: u64) -> Count {
        Count {
            prev: number,
            number: if number > self.prev {
                self.number + 1
            } else {
                self.number
            },
        }
    }

    fn number(&self) -> u64 {
        self.number
    }
}

struct Count3 {
    prev3: [u64; 3],
    number: u64,
}

impl Count3 {
    pub fn new() -> Count3 {
        Count3 {
            prev3: [100000; 3],
            number: 0,
        }
    }

    pub fn include(self, nums: [u64; 3]) -> Count3 {
        Count3 {
            prev3: nums,
            number: if Count3::sum(nums) > Count3::sum(self.prev3) {
                self.number + 1
            } else {
                self.number
            },
        }
    }

    fn sum(val3: [u64; 3]) -> u64 {
        return val3[0] + val3[1] + val3[2];
    }

    fn number(&self) -> u64 {
        self.number
    }
}

fn problem1(input: &'static str) -> u64 {
    input
        .lines()
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .fold(Count::new(), |count, number| count.include(number))
        .number()
}

fn problem2(input: &'static str) -> u64 {
    let input: Vec<_> = input
        .lines()
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
    (0..input.len() - 2)
        .into_iter()
        .map(|i| [input[i], input[i + 1], input[i + 2]])
        .fold(Count3::new(), |count, nums| count.include(nums))
        .number()
}

#[test]
fn problem1_test() {
    println!("{}", problem1(include_str!("day01.txt")))
}

#[test]
fn problem2_test() {
    println!("{}", problem2(include_str!("day01.txt")))
}
