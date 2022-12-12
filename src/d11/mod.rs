use std::{io::{Read, BufReader, BufRead}};

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    id: Option<usize>,
    items1: Vec<i64>,
    items2: Vec<i64>,
    operation: Option<Operation>,
    operand1: Option<i64>, // None means old
    operand2: Option<i64>, // None means old
    divisible_by: Option<i64>,
    true_monkey: Option<usize>,
    false_monkey: Option<usize>,
    inspected_count1: usize,
    inspected_count2: usize,
}

impl Monkey {
    fn new() -> Self {
        Self {
            id: None,
            items1: Vec::new(),
            items2: Vec::new(),
            operation: None,
            operand1: None,
            operand2: None,
            divisible_by: None,
            true_monkey: None,
            false_monkey: None,
            inspected_count1: 0,
            inspected_count2: 0,
        }
    }

    fn inspect(self: &mut Self, is_first: bool, worry: i64, divider: i64, max_divisible: i64) -> i64 {
        match is_first {
            true => {
                self.inspected_count1 += 1;
            }
            false => {
                self.inspected_count2 += 1;
            }
        };
        let op1 = match self.operand1 {
            Some(x) => x,
            None => worry,
        };
        let op2 = match self.operand2 {
            Some(x) => x,
            None => worry,
        };
        let result = match self.operation {
            Some(Operation::Plus) => op1 + op2,
            Some(Operation::Multiply) => op1 * op2,
            None => panic!("Operation missing"),
        };
        (result / divider) % max_divisible
    }

    fn worry_test(self: &Self, worry: i64) -> bool {
        match self.divisible_by {
            Some(divisible_by) => worry % divisible_by == 0,
            None => panic!("Divisible by missing"),
        }
    }

    fn add_item1(self: &mut Self, item: i64) {
        self.items1.push(item);
    }

    fn add_item2(self: &mut Self, item: i64) {
        self.items2.push(item);
    }
}

fn parse<R: Read>(input: BufReader<R>) -> Result<Vec<Monkey>, String> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey = Monkey::new();

    for line in input.lines() {
        match line {
            Ok(ln) => {
                let mut parts = ln.trim().split(' ');
                match parts.next() {
                    Some("Monkey") => match parts.next() {
                        Some(id) => {
                            match id.trim_end_matches(':').parse::<usize>() {
                                Ok(id_num) => {
                                    current_monkey.id = Some(id_num);
                                },
                                Err(_) => return Err(String::from("Monkey has invalid ID")),
                            };
                        },
                        None => return Err(format!("Monkey has now id")),
                    },
                    Some("Starting") => {
                        match parts.next() {
                            Some("items:") => {},
                            Some(_) => return Err(format!("Expected 'items:'")),
                            None => return Err(format!("Expected 'items:'")),
                        }
                        loop {
                            match parts.next() {
                                Some(text) => match text.trim_end_matches(',').parse::<i64>() {
                                    Ok(num) => {
                                        current_monkey.items1.push(num);
                                        current_monkey.items2.push(num);
                                    },
                                    Err(_) => return Err(format!("Not a number: {text}")),
                                },
                                None => break,
                            }
                        }
                    },
                    Some("Operation:") => {
                        match parts.next() {
                            Some("new") => {},
                            _ => return Err(format!("Expected 'new'")),
                        }
                        match parts.next() {
                            Some("=") => {},
                            _ => return Err(format!("Expected '='")),
                        }
                        current_monkey.operand1 = match parts.next() {
                            Some("old") => None,
                            Some(text) => match text.parse::<i64>() {
                                Ok(num) => Some(num),
                                Err(_) => return Err(String::from("Not a number")),
                            },
                            None => return Err(String::from("Number or 'old' is missing")),
                        };

                        current_monkey.operation = match parts.next() {
                            Some("+") => Some(Operation::Plus),
                            Some("*") => Some(Operation::Multiply),
                            _ => return Err(format!("Expected '+' or '*'")),
                        };

                        current_monkey.operand2 = match parts.next() {
                            Some("old") => None,
                            Some(text) => match text.parse::<i64>() {
                                Ok(num) => Some(num),
                                Err(_) => return Err(String::from("Not a number")),
                            },
                            None => return Err(String::from("Number or 'old' is missing")),
                        };
                    },
                    Some("Test:") => {
                        match parts.next() {
                            Some("divisible") => {},
                            _ => return Err(String::from("Expected 'divisible'")),
                        };
                        match parts.next() {
                            Some("by") => {},
                            _ => return Err(String::from("Expected 'by'")),
                        };

                        current_monkey.divisible_by = match parts.next() {
                            Some(text) => match text.parse::<i64>() {
                                Ok(num) => Some(num),
                                Err(_) => return Err(String::from("Not a number")),
                            },
                            None => return Err(String::from("Number is missing")),
                        };
                    },
                    Some("If") => match (parts.next(), parts.next(), parts.next(), parts.next(), parts.next()) {
                        (Some("true:"), Some("throw"), Some("to"), Some("monkey"), Some(monkey)) => {
                            match monkey.parse::<usize>() {
                                Ok(num) => {
                                    current_monkey.true_monkey = Some(num);
                                },
                                Err(_) => return Err(String::from("Invalid true monkey")),
                            };
                        },
                        (Some("false:"), Some("throw"), Some("to"), Some("monkey"), Some(monkey)) => {
                            match monkey.parse::<usize>() {
                                Ok(num) => {
                                    current_monkey.false_monkey = Some(num);
                                    monkeys.push(current_monkey);
                                    current_monkey = Monkey::new();
                                },
                                Err(_) => return Err(String::from("Invalid false monkey")),
                            };
                        },
                        _ => return Err(format!("Invalid If")),
                    },
                    Some("") => {},
                    Some(x) => return Err(format!("Invalid line start: {x}")),
                    None => return Err(format!("Invalid line {ln}")),
                }
            },
            Err(e) => return Err(format!("Unable to read line: {e}")),
        }
    }
    return Ok(monkeys);
}

fn run_for(monkeys: &mut Vec<Monkey>, is_first: bool, rounds: usize, factor: i64, max_divisble: i64) -> Result<usize, String> {
    for _round in 0..rounds {
        for monkey_id in 0..monkeys.len() {
            let items: Vec<i64> = match is_first {
                true => monkeys[monkey_id].items1.drain(..).collect(),
                false => monkeys[monkey_id].items2.drain(..).collect(),
            };

            for item in items.iter() {
                let worry_level = monkeys[monkey_id].inspect(is_first, *item, factor, max_divisble);
                let target_monkey = match monkeys[monkey_id].worry_test(worry_level) {
                    true => {
                        monkeys[monkey_id].true_monkey
                    },
                    false => {
                        monkeys[monkey_id].false_monkey
                    },
                };

                match target_monkey {
                    Some(id) => {
                        match is_first {
                            true => monkeys[id].add_item1(worry_level),
                            false => monkeys[id].add_item2(worry_level),
                        };
                    },
                    None => return Err(String::from("No target monkey")),
                }
            }
        }
    }

    let mut top1 = 0;
    let mut top2 = 0;
    for monkey in monkeys {
        let count = match is_first {
            true => monkey.inspected_count1,
            false => monkey.inspected_count2,
        };
        if count > top1 {
            top2 = top1;
            top1 = count;
            continue;
        }
        if count > top2 {
            top2 = count;
        }
    }
    return Ok(top1 * top2);
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut monkeys = match parse(input) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    // In second part, we would get out of i64 range, but given the actual worry level value is not used
    // we can just store reminder for all possible divisible_by values - mutliply all the divisible_by values
    let mut max_divisble = 1;
    for monkey in &monkeys {
        match monkey.divisible_by {
            Some(div) => {
                max_divisble *= div;
            },
            None => return Err(String::from("No divisible_by")),
        }
    }

    let result1 = match run_for(&mut monkeys, true, 20, 3, max_divisble) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let result2 = match run_for(&mut monkeys, false, 10000, 1, max_divisble) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    Ok((
        format!("{}", result1),
        format!("{}", result2),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d10_01() {
        let input = String::from("Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "10605",
        );
        assert_eq!(
            result.1,
            "2713310158",
        );
    }

    #[test]
    fn test_d10_final() {
        let f = std::fs::File::open("src/d11/input.txt").expect("No src/11/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "78678",
        );
        assert_eq!(
            result.1,
            "15333249714",
        );
    }
}
