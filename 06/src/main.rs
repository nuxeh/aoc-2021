use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2021))
        .day(Some(6))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        println!("{}", i);
        run(&i);
    }
}

fn run(input: &str) {
    let initial_state: Vec<u8> = input
        .trim()
        .split(",")
        .map(|f| {println!("{}", f); f})
        .map(|f| f.parse())
        .map(|f| {println!("{:?}", f); f})
        .flatten()
        .collect();

    println!("{:?}", initial_state);
}

