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
    let crabs: Vec<i16> = i
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
}
