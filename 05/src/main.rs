use aocf::Aoc;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_str(i: &str) -> Self {
        let c: Vec<i32> = i
            .replace(" -> ", ",")
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        let start = Point { x: c[0], y: c[1] };
        let end =  Point { x: c[2], y: c[3] };

        Self { start, end }
    }

    fn cast(self) -> Vec<Point> {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        println!("dx={} dy={}", dx, dy);

        // only horz
        //if !(dy == 0 || dx == 0) { return vec![]; }

        let (n, mut ix, mut iy) = if dy.abs() > dx.abs() {
            (dy.abs(), dx/dy, dy/dy)
        } else {
            (dx.abs(), dx/dx, dy/dx)
        };

        if dx > 0 { ix = ix.abs() } else { ix = -1 * ix.abs() }
        if dy > 0 { iy = iy.abs() } else { iy = -1 * iy.abs() }

        println!("n={} ix={} iy={}", n, ix, iy);

        (0..=n)
            .map(|n| {
                Point {
                    x: self.start.x + (n * ix),
                    y: self.start.y + (n * iy),
                }
            })
            .collect()
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2021))
        .day(Some(5))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        run(&i);
    }
}

fn run(i: &str) {
    let mut points: HashMap<Point, usize> = HashMap::new();

    let lines: Vec<Line> = i
        .lines()
        .map(|l| Line::from_str(l))
        .collect();

    println!("{:#?}", lines);

    for line in lines {
        println!("{:?}", line);
        let line_points = line.cast();
        println!("{:?}", line_points);

        for point in line_points {
            match points.get(&point) {
                Some(count) => points.insert(point, count+1),
                None => points.insert(point, 1),
            };
        };
    }

    println!("{:#?}", points);

    let gt2 = points
        .values()
        .filter(|value| **value >= 2)
        .count();

    println!("gt2={}", gt2);
} 

/*
  --> src/main.rs:97:32
   |
96 |             match points.get(&point) {
   |                   ------ immutable borrow occurs here
97 |                 Some(count) => points.insert(point, count+1),
   |                                ^^^^^^               ----- immutable borrow later used here
   |                                |
   |                                mutable borrow occurs here
   |
   = note: `#[warn(mutable_borrow_reservation_conflict)]` on by default
   = warning: this borrowing pattern was not meant to be accepted, and may become a hard error in the future
   = note: for more information, see issue #59159 <https://github.com/rust-lang/rust/issues/59159>

warning: `aoc0521` (bin "aoc0521") generated 3 warnings
 */
