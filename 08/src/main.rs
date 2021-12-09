use aocf::Aoc;
use std::collections::HashSet;

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
        println!("{:#?}", analyse(line));
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

fn subtract_from(first: &str, second: &str) -> String {
    first
        .chars()
        //.filter(|c| !second.contains(c))
        .collect()
}

fn analyse(line: &str) -> Vec<HashSet<char>> {
    let mut map: Vec<HashSet<char>> = vec![HashSet::new(); 10];

    let mut seven = get_char(line, 3).to_string();

    get_char(line, 2)
        .chars()
        .for_each(|c| {
            map[0].insert(c);
            map[1].insert(c);
            map[3].insert(c);
            map[4].insert(c);
            map[7].insert(c);
            map[8].insert(c);
            map[9].insert(c);
            seven.remove(seven.find(c).unwrap());
        });

    let top = seven.chars().nth(0).unwrap();
    map[0].insert(top);
    map[2].insert(top);
    map[3].insert(top);
    map[5].insert(top);
    map[6].insert(top);
    map[7].insert(top);
    map[8].insert(top);
    map[9].insert(top);

    let eight = get_char(line, 7);
    eight
        .chars()
        .for_each(|c| {
            map[8].insert(c);
        });

    let four = get_char(line, 4);
    four
        .chars()
        .for_each(|c| {
            map[4].insert(c);
        });

    println!("{} {}", seven, seven.len());

    map
}
