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
    for line in i.lines() {
        //println!("{:#?}", analyse(line));
        println!("{:#?}", reconstitute(line));
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
        .filter(|c| !second.contains(*c))
        .collect()
}

fn add(first: &str, second: &str) -> String {
    let mut v: Vec<char> = first
        .chars()
        .chain(second.chars())
        .collect();
    v.sort();
    v.dedup();
    v.iter().collect()
}

fn matches_exactly(first: &str, second: &str) -> bool {
    if second.len() == first.len() {
        matches(first, second)
    } else {
        false
    }
}

fn matches(first: &str, second: &str) -> bool {
    second
        .chars()
        .filter(|c| first.contains(*c))
        .count() == second.len()
}

fn reconstitute(line: &str) -> u16 {
    get_segment(line, 1)
        .iter()
        .map(|s| reconstitute_str(line, s))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn reconstitute_str(line: &str, s: &str) -> String {
    let eight = get_char(line, 7);
    let four = get_char(line, 4);
    let seven = get_char(line, 3);
    let one = get_char(line, 2);

    if matches_exactly(s, eight) { return "8".to_string() }    
    if matches_exactly(s, four) { return "4".to_string() }    
    if matches_exactly(s, seven) { return "7".to_string() }    
    if matches_exactly(s, one) { return "1".to_string() }    

    let map = analyse(line);

    if matches(s, &map[0].iter().collect::<String>()) { return "0".to_string() }
    if matches(s, &map[2].iter().collect::<String>()) { return "2".to_string() }
    if matches(s, &map[3].iter().collect::<String>()) { return "3".to_string() }
    if matches(s, &map[5].iter().collect::<String>()) { return "5".to_string() }
    if matches(s, &map[6].iter().collect::<String>()) { return "6".to_string() }
    if matches(s, &map[9].iter().collect::<String>()) { return "9".to_string() }

    "".to_string()
}

fn analyse(line: &str) -> Vec<HashSet<char>> {
    let mut map: Vec<HashSet<char>> = vec![HashSet::new(); 10];

    let seven = get_char(line, 3);
    let eight = get_char(line, 7);
    let four = get_char(line, 4);
    let one = get_char(line, 2);

    one
        .chars()
        .for_each(|c| {
            map[0].insert(c);
            map[1].insert(c);
            map[3].insert(c);
            map[4].insert(c);
            map[7].insert(c);
            map[8].insert(c);
            map[9].insert(c);
        });

    eight
        .chars()
        .for_each(|c| {
            map[8].insert(c);
        });

    four
        .chars()
        .for_each(|c| {
            map[4].insert(c);
            map[9].insert(c);
        });

    seven
        .chars()
        .for_each(|c| {
            map[7].insert(c);
        });

    let top = subtract_from(seven, one).chars().nth(0).unwrap();
    map[0].insert(top);
    map[2].insert(top);
    map[3].insert(top);
    map[5].insert(top);
    map[6].insert(top);
    map[7].insert(top);
    map[8].insert(top);
    map[9].insert(top);

    //println!("{} {} {}", seven, one, top);

    let almost_six = subtract_from(eight, seven);

    almost_six
        .chars()
        .for_each(|c| {
            map[6].insert(c);
        });

    let little_ell = subtract_from(&almost_six, four); 

    little_ell
        .chars()
        .for_each(|c| {
            map[0].insert(c);
            map[2].insert(c);
        });

    let higher_little_ell = subtract_from(four, one);

    higher_little_ell
        .chars()
        .for_each(|c| {
            map[5].insert(c);
            map[9].insert(c);
        });

    map
}
