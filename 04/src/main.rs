use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2021))
        .day(Some(4))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        run(&i);
    }
}

fn run(i: &str) {
    let seq: Vec<u8> = i
        .lines()
        .take(1)
        .collect::<String>()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();

    println!("seq={:?}", seq);

    let boards_stripped = i
        .replace("\n\n", "*")
        .replace("\n", " ")
        .replace("  ", " ")
        .replace("*", "\n");

    println!("boards=\n{}", boards_stripped);

    let boards: Vec<Vec<Vec<u8>>> = boards_stripped
        .lines()
        .skip(1) // sequence list
        .map(|l| l
            .split(" ")
            .map(|b| b.parse().unwrap_or(255))
            .collect::<Vec<u8>>()
            .chunks(5)
            .map(|n| n.into())
            .collect()
        )
        .collect();

    println!("boards=\n{:#?}", boards);

}
