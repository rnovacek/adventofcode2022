use std::{io::{Read, BufReader, BufRead}};

enum Command {
    Noop,
    Addx { x: i32 },
}

fn parse<R: Read>(input: BufReader<R>) -> impl Iterator<Item=Result<Command, String>> {
    input.lines().map(|line| {
        match line {
            Ok(ln) => {
                let mut parts = ln.split(' ');
                match parts.next() {
                    Some("noop") => Ok(Command::Noop),
                    Some("addx") => match parts.next() {
                        Some(num) => match num.parse::<i32>() {
                            Ok(x) => Ok(Command::Addx { x }),
                            Err(_) => Err(format!("addx param must be a number")),
                        }
                        None => Err(format!("Invalid addx param")),
                    },
                    Some(cmd) => Err(format!("Invalid command {cmd}")),
                    None => Err(format!("Invalid line {ln}")),
                }
            },
            Err(e) => Err(format!("Unable to read line: {e}")),
        }
    })
}

struct Display {
    d: String,
    checksum: i32,
    register: i32,
    cycle: i32,
}

const WIDTH: i32 = 40;
const CHECKSUM_OFFSET: i32 = 20;

impl Display {
    fn new() -> Self {
        Self {
            d: String::new(),
            checksum: 0i32,
            register: 1,
            cycle: 0,
        }
    }

    fn checksum_increase(&mut self) {
        if (self.cycle - CHECKSUM_OFFSET) % WIDTH == 0 {
            self.checksum += self.cycle * self.register;
        }
    }

    fn draw(&mut self) {
        println!("Cycle {}, register: {}", self.cycle, self.register);
        self.cycle += 1;

        let pos = self.cycle % WIDTH;

        let mut on = false;
        // FIXME: end of line handling is probably invalid
        if pos == self.register {
            on = true;
        } else if pos == self.register + 1 {
            on = true;
        } else if pos == self.register + 2 {
            on = true;
        }

        if on {
            self.d.push('#');
        } else {
            self.d.push('.');
        }

        if self.cycle % WIDTH == 0 {
            self.d.push('\n');
        }

        self.checksum_increase();

    }

    fn add(&mut self, num: i32) {
        self.register += num;
    }
}


pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut display = Display::new();

    for command in parse(input) {
        match command {
            Ok(cmd) => {
                match cmd {
                    Command::Noop => {
                        display.draw();
                    }
                    Command::Addx { x } => {
                        display.draw();
                        display.draw();
                        display.add(x);
                    }
                };
            },
            Err(e) => return Err(e),
        }
    }

    Ok((
        format!("{}", display.checksum),
        format!("\n{}", display.d),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d10_01() {
        let input = String::from("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "13140",
        );
        assert_eq!(
            result.1,
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......###.
#######.......#######.......#######.....
",
        );
    }

    #[test]
    fn test_d10_final() {
        let f = std::fs::File::open("src/d10/input.txt").expect("No src/10/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "14420",
        );
        assert_eq!(
            result.1,
            "
###...##..#....###..###..####..##..#..#.
#..#.#..#.#....#..#.#..#....#.#..#.#..##
#..#.#....#....#..#.###....#..#..#.#..#.
###..#.##.#....###..#..#..#...####.#..##
#.#..#..#.#....#.#..#..#.#....#..#.#..##
#..#..###.####.#..#.###..####.#..#..##..
",
        );
    }
}
