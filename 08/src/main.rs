use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2021))
        .day(Some(8))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        run(&i);
    }
}

struct Line<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

fn run(i: &str) {
    let lines: Vec<Vec<&str>> = i
        .trim()
        .lines()
        .map(|l| l.split(" | ").collect())
        .collect();

    println!("{:#?}", lines);

    let outputs: Vec<&&str> = lines
        .iter()
        .map(|l| l.get(1))
        .flatten()
        .collect();

    println!("{:#?}", outputs);

    let outputs_1478: Vec<usize> = outputs
        .iter()
        .map(|l| l
            .split(' ')
            .map(|o| o.len())
        )
        .flatten()
        .filter(|o| [2, 3, 4, 7].contains(o))
        .collect();

    println!("{:#?}", outputs_1478.iter().count());



}
