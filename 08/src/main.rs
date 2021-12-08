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

    // Part 2

    println!("{:?}", get_char("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe", 2));

    for line in i.lines() {
        println!("{:?}", get_char(line, 2));
        println!("{:?}", analyse(line));
    }
}

fn get_char(line: &str, len: usize) -> &str {
    get_segment(line, 0)
        .iter()
        .filter(|c| c.len() == len)
        .nth(0)
        .unwrap()
}

fn get_segment(line: &str, n: usize) -> Vec<&str> {
    line
        .split(" | ")
        .nth(n)
        .unwrap()
        .split(' ')
        .collect()
}

fn analyse(line: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![vec![]; 10];

    get_char(line, 2)
        .chars()
        .for_each(|c| {
            map[0].push(c);
            map[3].push(c);
            map[4].push(c);
            map[7].push(c);
            map[8].push(c);
            map[9].push(c);
        });

    map
}
