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
    let mut initial_state: Vec<i8> = input
        .trim()
        .split(",")
        .map(|f| f.parse())
        .flatten()
        .collect();

    println!("{:?}", initial_state);

    let ipop: f64 = initial_state.iter().count() as f64;

    for day in 0..80 {
        let mut add = 0;

        initial_state = initial_state
            .iter()
            .map(|fish| {
                let mut newfish = *fish - 1;

                if newfish == -1 {
                    add += 1;
                    newfish = 6;
                }

                newfish
            })
            .collect();

        (0..add).for_each(|_| initial_state.push(8));

        let pop: f64 = initial_state.iter().count() as f64;
        let k = (pop/ipop).ln() / day as f64;

        //println!("{:2} (k={}) -> {:?}", day, k, initial_state);
        println!("{:2} (k={})", day, k);
    }

    println!("{}", initial_state.iter().count());
}

