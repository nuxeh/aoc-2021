use aocf::Aoc;
use regex::Regex;

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

    let i_boards: String = i
        .lines()
        .skip(2)
        .join('\n')
        .collect::<String>();
    println!("{:?}", i_boards);

    let re = Regex::new(r"\d\n").unwrap();
    let boards_stripped: String = re.replace_all(&i_boards, "").to_string();

    println!("boards=\n{}", boards_stripped);
}
