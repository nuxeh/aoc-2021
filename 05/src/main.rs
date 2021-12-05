use aocf::Aoc;
use std::collections::HashMap;

#[derive(Debug)]
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

      let p1 = Point { x: c[0], y: c[1] };
      let p2 =  Point { x: c[2], y: c[3] };

      let (start, end) = if p1.x > p2.x {
          (p2, p1)
      } else {
          (p1, p2)
      };

      Self { start, end }
  }

  fn cast(self) -> Vec<Point> {
      let dx = self.end.x - self.start.x;
      let dy = self.end.y - self.start.y;

      (self.start.x..=self.end.x)
          .map(|x| {
              Point {
                  x: x,
                  y: self.start.y + (dy * ((x - self.start.x) / dx)),
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
    let lines: Vec<Line> = i
        .lines()
        .map(|l| Line::from_str(l))
        .collect();

    println!("{:#?}", lines);

    for line in lines {
        println!("{:?}", line);
        println!("{:?}", line.cast());
    }
} 
