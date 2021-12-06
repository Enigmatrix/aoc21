const INPUT: &'static str = include_str!("day06.txt");

fn problem1() -> usize {
    let mut fish: Vec<i64> = INPUT.split(',').map(|s| s.parse().unwrap()).collect();
    for _ in 0..80 {
        let mut new_fish = Vec::new();
        for f in fish.iter_mut() {
            *f -= 1;
            if *f < 0 {
                new_fish.push(8);
                *f = 6;
            }
        }
        fish.append(&mut new_fish);
    }
    fish.len()
}

fn problem2() -> i64 {
    let mut fish = [0; 9];
    for f in INPUT.split(',').map(|s| s.parse::<usize>().unwrap()) {
        fish[f] += 1;
    }
    for i in 0..256 {
        let front = fish[i % 9];
        fish[(i + 7) % 9] += front;
        // fish[(i + 9) % 9] = front <--- this is unnecessary
    }
    fish.iter().sum()
}

#[test]
fn problem1_test() {
    println!("{}", problem1())
}

#[test]
fn problem2_test() {
    println!("{}", problem2())
}
