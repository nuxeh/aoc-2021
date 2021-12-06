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
    let mut initial_state: Vec<u8> = input
        .trim()
        .split(",")
        .map(|f| f.parse())
        .flatten()
        .collect();

    println!("{:?}", initial_state);

    for day in 0..18 {
        let mut add = 0;

        initial_state = initial_state
            .iter()
            .map(|fish| {
                if *fish > 0 {
                    let mut newfish = *fish - 1;
                    if newfish == 0 {
                        add += 1;
                        newfish = 6;
                    }
                    newfish
                } else {
                    *fish
                }
            })
            .collect();

        (0..add).for_each(|_| initial_state.push(8));

        println!("{:2} -> {:?}", day, initial_state);
    }

    println!("{}", initial_state.iter().count());
}

