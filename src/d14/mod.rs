use std::{io::{Read, BufReader, Bytes}, collections::{HashMap}, cmp, hash::{Hash}};

#[derive(Debug, Clone)]
enum Content {
    Rock,
    Sand,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position {
    x: isize,
    y: isize,
}

struct Map {
    map: HashMap<Position, Content>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
    with_floor: bool,
}

impl Map {
    fn insert(&mut self, x: isize, y: isize, content: Content) {
        let p = Position { x, y };
        self.map.insert(p, content);
        self.min_x = cmp::min(self.min_x, x);
        self.max_x = cmp::max(self.max_x, x);
        self.min_y = cmp::min(self.min_y, y);
        self.max_y = cmp::max(self.max_y, y);
    }

    fn parse<R: Read>(input: &mut Bytes<BufReader<R>>) -> Result<Map, String> {
        let mut current = String::with_capacity(4);

        let mut positions: Vec<isize> = Vec::with_capacity(4);

        let mut map = Map {
            map: HashMap::new(),
            min_x: isize::MAX,
            min_y: isize::MAX,
            max_x: 0isize,
            max_y: 0isize,
            with_floor: false,
        };

        for maybe_byte in input {
            match maybe_byte {
                Ok(b'-') => {},
                Ok(b'>') => {},
                Ok(b',') | Ok(b' ') | Ok(b'\n') => {
                    if !current.is_empty() {
                        positions.push(
                            match current.parse::<isize>() {
                                Ok(n) => n,
                                Err(e) => return Err(format!("Wrong number format '{}': {}", current, e)),
                            }
                        );

                        if positions.len() == 4 {
                            if positions[0] != positions[2] {
                                assert_eq!(positions[1], positions[3]);
                                for i in cmp::min(positions[0], positions[2])..=cmp::max(positions[0], positions[2]) {
                                    map.insert(i, positions[1], Content::Rock);
                                }
                            }

                            if positions[1] != positions[3] {
                                assert_eq!(positions[0], positions[2]);
                                for i in cmp::min(positions[1], positions[3])..=cmp::max(positions[1], positions[3]) {
                                    map.insert(positions[0], i, Content::Rock);
                                }
                            }

                            if maybe_byte.unwrap() == b'\n' {
                                positions.clear();
                            } else {
                                positions.remove(0);
                                positions.remove(0);
                            }
                        }
                    }
                    current.clear();
                },
                Ok(ch) => {
                    current.push(ch as char);
                },
                Err(e) => return Err(format!("Unable to read: {}", e)),
            }
        }
        Ok(map)
    }

    fn _print(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                print!("{}", match self.map.get(&Position { x, y }) {
                    Some(Content::Rock) => "#",
                    Some(Content::Sand) => "o",
                    None => ".",
                });
            }
            println!("");
        }
    }

    fn is_empty(&self, pos: &Position) -> bool {
        match self.map.get(pos) {
            Some(Content::Rock) | Some(Content::Sand) => false,
            None => {
                if self.with_floor && pos.y >= self.max_y {
                     // Hits the floor
                    return false;
                }
                // Empty space
                return true;
            }
        }
    }

    fn drop_sand(&self, start: &Position) -> Option<Position> {
        let mut pos = Position { x: start.x, y: start.y };
        while pos.y <= self.max_y + 1 {
            // Try down
            if self.is_empty(&Position { x: pos.x, y: pos.y + 1 }) {
                pos.y += 1;
                continue;
            }
            if self.is_empty(&Position { x: pos.x - 1, y: pos.y + 1 }) {
                pos.y += 1;
                pos.x -= 1;
                continue;
            }
            if self.is_empty(&Position { x: pos.x + 1, y: pos.y + 1 }) {
                pos.y += 1;
                pos.x += 1;
                continue;
            }

            return Some(pos);
        }
        None
    }

    fn simulate_sand(&mut self, start: &Position) -> i32 {
        let mut sand_count = 0;
        loop {
            match self.drop_sand(start) {
                Some(pos) => {
                    self.insert(pos.x, pos.y, Content::Sand);
                    sand_count += 1;

                    if &pos == start {
                        // start is full of sand
                        break;
                    }
                }
                None => break,
            }
        }
        return sand_count;
    }
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let start = Position { x: 500, y: 0};
    let mut map = Map::parse(&mut input.bytes())?;
    // map._print();

    let mut map2 = Map {
        map: map.map.clone(),
        min_x: map.min_x,
        max_x: map.max_x,
        min_y: map.min_y,
        max_y: map.max_y + 2,
        with_floor: true,
    };

    let sand_count1 = map.simulate_sand(&start);
    // map._print();
    // println!("\n+++++++++++++++++++++++++++++++\n");

    let sand_count2 = map2.simulate_sand(&start);
    // map2._print();

    Ok((
        format!("{}", sand_count1),
        format!("{}", sand_count2),
    ))
}

#[cfg(test)]
mod test {
    use std::io::BufReader;

    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d14_01() {
        let input = String::from("498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "24",
        );
        assert_eq!(
            result.1,
            "93",
        );
    }

    #[test]
    fn test_d14_final() {
        let f = std::fs::File::open("src/d14/input.txt").expect("No src/14/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "964",
        );
        assert_eq!(
            result.1,
            "32041",
        );
    }
}
