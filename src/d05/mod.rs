use std::{io::{BufReader, BufRead, Read}};

enum Section {
    STACKS,
    BLANK,
    OPERATIONS,
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut section = Section::STACKS;
    let mut stacks_9000: Vec<Vec<char>> = Vec::new();
    let mut stacks_9001: Vec<Vec<char>> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();
    for line in input.lines() {
        match line {
            Ok(text) => {
                match section {
                    Section::STACKS => {
                        if text.chars().nth(1).unwrap_or('?').is_numeric() {
                            section = Section::BLANK;
                            continue;
                        }

                        let mut index = 0;
                        let mut chars = text.chars();
                        loop {
                            chars.next();
                            let content = match chars.next() {
                                Some(x) => x,
                                None => break,
                            };
                            chars.next();
                            chars.next();

                            if stacks_9000.len() <= index {
                                stacks_9000.push(Vec::new());
                                stacks_9001.push(Vec::new());
                            }

                            if content != ' ' {
                                stacks_9000[index].insert(0, content);
                                stacks_9001[index].insert(0, content);
                            }
                            index += 1;
                        }
                    },
                    Section::BLANK => {
                        section = Section::OPERATIONS;
                    },
                    Section::OPERATIONS => {
                        let mut parts = text.split(' ');
                        match parts.next() {
                            Some("move") => "move",
                            Some(x) => return Err(format!("Invalid token {}", x)),
                            None => return Err(format!("Invalid line {}, expected move", text)),
                        };
                        let count = match parts.next() {
                            Some(num) => match num.parse::<usize>() {
                                Ok(x) => x,
                                Err(e) => return Err(format!("Invalid number on line {}: {}", text, e)),
                            },
                            None => return Err(format!("Invalid line {}, expected number", text)),
                        };
                        match parts.next() {
                            Some("from") => "from",
                            Some(x) => return Err(format!("Invalid token {}", x)),
                            None => return Err(format!("Invalid line {}, expected from", text)),
                        };
                        let from = match parts.next() {
                            Some(num) => match num.parse::<usize>() {
                                Ok(x) => x,
                                Err(e) => return Err(format!("Invalid number on line {}: {}", text, e)),
                            },
                            None => return Err(format!("Invalid line {}, expected number", text)),
                        };
                        match parts.next() {
                            Some("to") => "to",
                            Some(x) => return Err(format!("Invalid token {}", x)),
                            None => return Err(format!("Invalid line {}, expected from", text)),
                        };
                        let to = match parts.next() {
                            Some(num) => match num.parse::<usize>() {
                                Ok(x) => x,
                                Err(e) => return Err(format!("Invalid number on line {}: {}", text, e)),
                            },
                            None => return Err(format!("Invalid line {}, expected number", text)),
                        };

                        for _ in 0..count {
                            match stacks_9000[from - 1].pop() {
                                Some(el) => stacks_9000[to - 1].push(el),
                                None => return Err(format!("Line {} attempted to pop from empty list", text)),
                            }
                            match stacks_9001[from - 1].pop() {
                                Some(el) => buffer.push(el),
                                None => return Err(format!("Line {} attempted to pop from empty list", text)),
                            }
                        }
                        loop {
                            match buffer.pop() {
                                Some(el) => stacks_9001[to - 1].push(el),
                                None => break,
                            }
                        }
                    }
                }
            }
            Err(e) => return Err(e.to_string())
        }
    }

    let mut result1 = String::new();
    for stack in stacks_9000 {
        match stack.last() {
            Some(val) => result1.push(*val),
            None => return Err(format!("Stack empty")),
        }
    }

    let mut result2 = String::new();
    for stack in stacks_9001 {
        match stack.last() {
            Some(val) => result2.push(*val),
            None => return Err(format!("Stack empty")),
        }
    }

    return Ok((result1, result2));
}