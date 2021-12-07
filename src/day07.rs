const INPUT: &'static str = include_str!("day07.txt");

fn problem1() -> i64 {
    let pos: Vec<i64> = INPUT.split(',').map(|s| s.parse().unwrap()).collect();
    (0..2000)
        .map(|i| {
            pos.iter()
                .map(|p| (p - i).abs())
                .sum()
        })
        .min()
        .unwrap()
}

fn problem2() -> i64 {
    let pos: Vec<i64> = INPUT.split(',').map(|s| s.parse().unwrap()).collect();
    (0..2000)
        .map(|i| {
            pos.iter()
                .map(|p| (p - i).abs())
                .map(|n| n * (n + 1) / 2)
                .sum()
        })
        .min()
        .unwrap()
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
