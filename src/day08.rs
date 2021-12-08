const INPUT: &'static str = include_str!("day08.txt");

struct Entry {
    input: Vec<String>,
    output: Vec<String>,
}

impl From<&str> for Entry {
    fn from(line: &str) -> Self {
        let (ins, outs) = line.split_once(" | ").unwrap();
        let input: Vec<String> = ins.split_whitespace().map(Entry::sort_str).collect();
        let output: Vec<String> = outs.split_whitespace().map(Entry::sort_str).collect();
        Entry { input, output }
    }
}

impl Entry {
    fn sort_str(s: &str) -> String {
        let mut sorted = s.chars().collect::<Vec<char>>();
        sorted.sort();
        sorted.iter().collect()
    }

    fn map_to(self) -> i64 {
        let mut _2_3_5 = Vec::new();
        let mut _0_6_9 = Vec::new();
        let mut nums = vec![String::new(); 10];

        self.input.iter().for_each(|s| {
            match s.len() {
                2 => nums[1] = s.clone(),    // 1
                4 => nums[4] = s.clone(),    // 4
                3 => nums[7] = s.clone(),    // 7
                7 => nums[8] = s.clone(),    // 8
                5 => _2_3_5.push(s.clone()), // 2,3,5
                6 => _0_6_9.push(s.clone()), // 0,6,9
                _ => unreachable!(),
            }
        });

        let a = rmv_subseq(&nums[7], &nums[1]);
        let bd = rmv_subseq(&nums[4], &nums[1]);
        let eg = rmv_subseq(&rmv_subseq(&nums[8], &nums[4]), &a);

        nums[2] = _2_3_5
            .iter()
            .filter(|s| contains_all(s, &eg))
            .next()
            .unwrap()
            .clone();
        nums[5] = _2_3_5
            .iter()
            .filter(|s| contains_all(s, &bd))
            .next()
            .unwrap()
            .clone();
        nums[3] = _2_3_5
            .iter()
            .filter(|&s| s != &nums[2] && s != &nums[5])
            .next()
            .unwrap()
            .clone();

        nums[0] = _0_6_9
            .iter()
            .filter(|s| !contains_all(s, &bd))
            .next()
            .unwrap()
            .clone();
        nums[6] = _0_6_9
            .iter()
            .filter(|&s| s != &nums[0])
            .filter(|s| contains_all(s, &eg))
            .next()
            .unwrap()
            .clone();
        nums[9] = _0_6_9
            .iter()
            .filter(|&s| s != &nums[0] && s != &nums[6])
            .next()
            .unwrap()
            .clone();

        self.output
            .iter()
            .map(|o| nums.iter().position(|s| s == o).unwrap() as i64)
            .fold(0, |a, e| a * 10 + e)
    }
}

fn rmv_subseq(one: &String, two: &String) -> String {
    one.chars()
        .filter(|s| !two.contains(&s.to_string()))
        .collect()
}

fn contains_all(one: &str, two: &str) -> bool {
    two.chars().all(|s| one.contains(&s.to_string()))
}

fn problem1() -> usize {
    let pos: Vec<Vec<&str>> = INPUT
        .lines()
        .map(|s| s.split_once("|").unwrap().1.split_whitespace().collect())
        .collect();
    pos.iter()
        .map(|s| {
            s.iter()
                .filter(|y| y.len() == 2 || y.len() == 4 || y.len() == 3 || y.len() == 7)
                .count()
        })
        .sum()
}

fn problem2() -> i64 {
    INPUT.lines().map(Entry::from).map(Entry::map_to).sum()
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
