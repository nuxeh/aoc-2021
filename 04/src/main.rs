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

    let mut boards: Vec<Vec<Vec<Option<u8>>>> = boards_stripped
        .lines()
        .skip(1) // sequence list
        .map(|l| l
            .split(" ")
            .filter(|n| !n.is_empty()) // if start of row is single digit
            .map(|n| Some(n.parse().unwrap()))
            .collect::<Vec<Option<u8>>>()
            .chunks(5)
            .map(|r| r.into())
            .collect()
        )
        .collect();

    println!("boards=\n{:#?}", boards);

    seq
        .iter()
        .for_each(|d| {
            println!("{:?}", boards);
            draw(&mut boards, d.clone());
        });
}

/// mutate
fn draw(boards: &mut Vec<Vec<Vec<Option<u8>>>>, draw: u8) {
    for board in boards.iter_mut() {
        for line in board.iter_mut() {
            for num in line.iter_mut() {
                if num == &mut Some(draw) {
                    *num = None;
                }
            }
        }
        test_complete(&board);
    }
}

fn test_complete(board: &Vec<Vec<Option<u8>>>) -> Option<u32> {
    for line in board.iter() {
        // check for complete rows
        let complete = line
            .iter()
            .fold(0, |mut acc, n| {
                if n == &None { acc += 1 }
                acc
            });

        if complete == 5 {
            let sum = sum_board(board);
            println!("COMPLETE ROW sum={}", sum);
            return Some(sum);
        }
    }
        /*
        for num in line.iter_mut() {
    // check for complete columns
    let complete: Vec<u8> = board
        .iter()
        .fold(Vec::new(), |acc, l| {
            l
                .iter()
                .enumerate()
                .for_each(
        });
        */

    None
}

fn sum_board(board: &Vec<Vec<Option<u8>>>) -> u32 {
    255
}
