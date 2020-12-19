mod map;

use map::{Map, Row, Slope};

fn main() -> Result<(), ()> {
  let mut map = parse_map_from_file("./input.txt")?;

  part1(&mut map);
  part2(&mut map);
  Ok(())
}

fn part1(map: &mut Map) {
  let slope = Slope::new(3, 1);

  let count = map.count_trees(slope);

  println!("[part 1] the number of trees encountered is {}", count);

}

fn part2(map: &mut Map) {
  let slopes: Vec<Slope> = vec![
    Slope::new(1, 1),
    Slope::new(3, 1),
    Slope::new(5, 1),
    Slope::new(7, 1),
    Slope::new(1, 2),
  ];

  let count: usize = slopes.iter().map(|&s| map.count_trees(s)).product();

  println!("[part 2] the product of trees encountered is {}", count);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// copied and pasted from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse_map(v: &[String]) -> Map {
  let rows: Vec<Row> = v.iter().map(|l| Row::from(&l[..])).collect();

  Map::new(rows)
}

fn parse_map_from_file(filename: &str) -> Result<Map, ()> {
  read_lines(filename)
    .map(|lines| {
      let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
      parse_map(&lines)
    })
    .or(Err(()))
}
