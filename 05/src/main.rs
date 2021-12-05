use aocf::Aoc;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
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

      let (n, ix, iy) = if dy.abs() > dx.abs() {
          (dy.abs(), dx/dy, dy/dy)
      } else {
          (dx.abs(), dx/dx, dy/dx)
      };

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
    let points: HashMap<Point, usize> = HashMap::new();

    let lines: Vec<Line> = i
        .lines()
        .map(|l| Line::from_str(l))
        .collect();

    println!("{:#?}", lines);

    for line in lines {
        println!("{:?}", line);
        println!("{:?}", line.cast());

        line
            .cast()
            .for_each(|point| {
                match points.get(point) {
                    Some(count) => points.insert(*point, count+1),
                    None => points.insert(*point, 1),
                };
            });
    }
} 
