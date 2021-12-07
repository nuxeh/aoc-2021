use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2021))
        .day(Some(7))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        run(&i);
    }
}

fn run(i: &str) {
    let crabs: Vec<i32> = i
        .trim()
        .split(",")
        .map(|c| c.parse())
        .flatten()
        .collect();

    println!("{:?}", crabs);
    println!("n_crabs={}", crabs.iter().count());
    let max = crabs.iter().max().unwrap();
    println!("max={:?}", max);
    println!("min={:?}", crabs.iter().min());

    let costs: Vec<_> = (0..=*max)
        .map(|pos| {
            crabs
                .iter()
                .fold(0, |acc, c| acc + (pos - c).abs())
        })
        .collect();

    println!("{:?}", costs);
    println!("min_fuel={:?}", costs.iter().min());

    part2(&crabs, *max);
}

fn part2(crabs: &Vec<i32>, max: i32) {
    let costs: Vec<_> = (0..=max)
        .map(|pos| {
            crabs
                .iter()
                .fold(0, |acc, c| acc + exp_dist((pos - c).abs()))
        })
        .collect();

    println!("{:?}", costs);
    println!("min_fuel={:?}", costs.iter().min());
}

fn exp_dist(x: i32) -> i32 {
    match x {
        0 | 1 => 1,
        _ => exp_dist(x - 1) + x,
    }
}
