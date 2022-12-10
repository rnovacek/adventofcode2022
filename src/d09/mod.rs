use std::{io::{Read, BufReader, BufRead}, collections::HashSet, hash::{Hash, Hasher}};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}


impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Position {
    fn move_dir(&self, dir: &Direction) -> Position {
        match dir {
            Direction::Left => Position { x: self.x - 1, y: self.y },
            Direction::Right => Position { x: self.x + 1, y: self.y },
            Direction::Down => Position { x: self.x, y: self.y + 1 },
            Direction::Up => Position { x: self.x, y: self.y - 1 },
        }
    }
}

fn move_tail(head: &Position, tail: &Position) -> Position {
    let diff_x = head.x - tail.x;
    let diff_y = head.y - tail.y;

    if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
        return Position { x: tail.x, y: tail.y };
    }

    Position {
        x: match diff_x {
            x if x < 0 => tail.x - 1,
            x if x > 0 => tail.x + 1,
            _ => tail.x,
        },
        y: match diff_y {
            y if y < 0 => tail.y - 1,
            y if y > 0 => tail.y + 1,
            _ => tail.y,
        },
    }
}

const TAIL_COUNT: usize = 9;

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut visited2: HashSet<Position> = HashSet::new();
    let mut visited10: HashSet<Position> = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tails: Vec<Position> = Vec::new();

    for _ in 0..TAIL_COUNT {
        tails.push(Position { x: 0, y: 0 });
    }

    for line in input.lines() {
        match line {
            Ok(ln) => {
                let mut parts = ln.split(' ');
                let dir = match parts.next() {
                    Some("L") => Direction::Left,
                    Some("R") => Direction::Right,
                    Some("D") => Direction::Down,
                    Some("U") => Direction::Up,
                    Some(x) => return Err(format!("Invalid direction: {}", x)),
                    None => return Err(format!("Invalid line: {}", ln)),
                };
                let count = match parts.next() {
                    Some(num) => {
                        match num.parse::<u32>() {
                            Ok(count) => count,
                            Err(_) => return Err(format!("Not a number on line: {}", ln)),
                        }
                    },
                    None => return Err(format!("Invalid line: {}", ln)),
                };

                for _ in 0..count {
                    // println!("{:?} Head {:?}, Tails: {:?}", dir, head, tails);
                    head = head.move_dir(&dir);
                    let mut last_tail = &head;
                    for i in 0..TAIL_COUNT {
                        tails[i] = move_tail(last_tail, &tails[i]);
                        last_tail = &tails[i];
                    }

                    visited2.insert(Position { x: tails[0].x, y: tails[0].y });
                    visited10.insert(Position { x: tails[8].x, y: tails[8].y });
                }
            },
            Err(e) => return Err(format!("Unable to read line by line: {}", e.to_string())),
        }
    }
    Ok((
        format!("{}", visited2.len()),
        format!("{}", visited10.len()),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d09_01() {
        let input = String::from("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "13",
        );
        assert_eq!(
            result.1,
            "1",
        );
    }

    #[test]
    fn test_d09_02() {
        let input = String::from("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "88",
        );
        assert_eq!(
            result.1,
            "36",
        );
    }
    #[test]
    fn test_d09_final() {
        let f = std::fs::File::open("src/d09/input.txt").expect("No src/d09/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "6745",
        );
        assert_eq!(
            result.1,
            "2793",
        );
    }
}
