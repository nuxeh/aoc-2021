use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some({{ year }}))
        .day(Some({{ day }}))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        run(&i);
    }
}

fn run(i: &str) {
}
