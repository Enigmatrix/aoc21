const INPUT: &'static str = include_str!("day03.txt");

fn count_0(inp: &Vec<&str>, idx: usize) -> usize {
    return inp
        .iter()
        .filter(|&&s| s.as_bytes()[idx] == '0' as u8)
        .count();
}

fn problem1() -> i64 {
    let input: Vec<_> = INPUT.lines().collect();
    let bits = input[0].len();
    let len = input.len();

    let gamma = (0..bits)
        .map(|i| {
            if count_0(&input, i) > len / 2 {
                "0"
            } else {
                "1"
            }
        })
        .collect::<Vec<_>>()
        .join("");

    let eps = gamma
        .as_bytes()
        .iter()
        .map(|&c| if c == '0' as u8 { "1" } else { "0" }) // toggle
        .collect::<Vec<_>>()
        .join("");

    i64::from_str_radix(&gamma, 2).unwrap() * i64::from_str_radix(&eps, 2).unwrap()
}

fn problem2() -> i64 {
    let input: Vec<_> = INPUT.lines().collect();

    let mut input_copy = input.clone();
    let mut idx = 0;
    let oxy = loop {
        if input_copy.len() == 1 {
            break input_copy[0];
        }
        let len = input_copy.len();
        let count1 = len - count_0(&input_copy, idx);
        if count1 >= len / 2 {
            input_copy = input_copy
                .into_iter()
                .filter(|&c| c.as_bytes()[idx] == '1' as u8)
                .collect();
        } else {
            input_copy = input_copy
                .into_iter()
                .filter(|&c| c.as_bytes()[idx] == '0' as u8)
                .collect();
        }

        idx += 1;
    };

    let mut input_copy = input.clone();
    let mut idx = 0;
    let co2 = loop {
        if input_copy.len() == 1 {
            break input_copy[0];
        }
        let len = input_copy.len();
        let count0 = count_0(&input_copy, idx);

        if count0 <= len / 2 {
            input_copy = input_copy
                .into_iter()
                .filter(|&c| c.as_bytes()[idx] == '0' as u8)
                .collect();
        } else {
            input_copy = input_copy
                .into_iter()
                .filter(|&c| c.as_bytes()[idx] == '1' as u8)
                .collect();
        }

        idx += 1;
    };

    i64::from_str_radix(&oxy, 2).unwrap() * i64::from_str_radix(&co2, 2).unwrap()
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
