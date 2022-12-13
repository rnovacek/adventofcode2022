use std::{io::{Read, BufReader, Bytes}, cmp::Ordering};

#[derive(Debug)]
enum Item {
    Array { items: Vec<Item> },
    Number { item: i32 },
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Number { item: a }, Item::Number { item: b }) => {
                return a.cmp(b);
            },
            (Item::Array { items: a }, Item::Array { items: b }) => {
                let mut index = 0;
                loop {
                    let a_el = match a.get(index) {
                        Some(a) => a,
                        None => {
                            return match b.get(index) {
                                Some(_) => Ordering::Less,
                                None => Ordering::Equal,
                            };
                        }
                    };
                    match b.get(index) {
                        Some(b_el) => match a_el.cmp(b_el) {
                            Ordering::Equal => {},
                            ord => {
                                return ord;
                            },
                        },
                        None => {
                            return Ordering::Greater;
                        }
                    }
                    index += 1;
                }
            },
            (Item::Number { item: a }, b @ Item::Array { items: _ }) => {
                let arr = Item::Array { items: vec![Item::Number { item: *a }] };
                return arr.cmp(b);
            },
            (a @ Item::Array { items: _ }, Item::Number { item: b }) => {
                let arr = Item::Array { items: vec![Item::Number { item: *b }] };
                return a.cmp(&arr);
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {
}

fn parse_into(str: &String, into: &mut Vec<Item>) -> Result<(), String> {
    if str.len() == 0 {
        return Ok(());
    }
    match str.parse::<i32>() {
        Ok(num) => {
            into.push(Item::Number { item: num });
            Ok(())
        }
        Err(e) => Err(format!("Unable to parse {}: {}", str, e.to_string())),
    }
}

fn process<R: Read>(iter: &mut Bytes<BufReader<R>>, into: &mut Vec<Item>) -> Result<(), String> {
    let mut current = String::new();

    loop {
        match iter.next() {
            Some(Ok(ch)) => {
                match ch {
                    b'\n' => {
                        assert!(current.len() == 0);
                        return parse_into(&current, into);
                    },
                    b']' => {
                        return parse_into(&current, into);
                    },
                    b'[' => {
                        let mut v: Vec<Item> = Vec::new();
                        process(iter, &mut v)?;
                        into.push(Item::Array { items: v });
                    },
                    b',' => {
                        parse_into(&current, into)?;
                        current.clear();
                    },
                    x => {
                        current.push(x as char);
                    }
                }
            },
            Some(Err(e)) => return Err(format!("Error: {}", e.to_string())),
            None => {
                return parse_into(&current, into);
            },
        }
    }
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut iter = input.bytes();
    let mut count = 0usize;
    let mut index = 1usize;
    let mut done = false;

    let marker1 = Item::Array { items: vec![
        Item::Array { items: vec![
            Item::Number { item: 2 },
        ]},
    ]};
    let marker2 = Item::Array { items: vec![
        Item::Array { items: vec![
            Item::Number { item: 6 },
        ]},
    ]};

    let mut all: Vec<Item> = vec![
        Item::Array { items: vec![
            Item::Array { items: vec![
                Item::Number { item: 2 },
            ]},
        ]},
        Item::Array { items: vec![
            Item::Array { items: vec![
                Item::Number { item: 6 },
            ]},
        ]},
    ];

    while !done {
        let mut first: Vec<Item> = Vec::new();
        process(&mut iter, &mut first)?;

        let mut second: Vec<Item> = Vec::new();
        process(&mut iter, &mut second)?;

        match iter.next() {
            Some(Ok(b'\n')) => {},
            Some(Ok(ch)) => return Err(format!("Expected new line, '{}' found", ch as char)),
            Some(Err(e)) => return Err(format!("Input error: {}", e.to_string())),
            None => {
                done = true;
            },
        }

        if first.le(&second) {
            count += index;
        }

        all.push(Item::Array { items: first });
        all.push(Item::Array { items: second });

        index += 1;
    }

    all.sort();

    let mut position1 = 0usize;
    let mut position2 = 0usize;
    let m1 = &marker1;
    for (index, item) in all.iter().enumerate() {
        if item.eq(&m1) {
            position1 = index + 1;
        }
        if item.eq(&marker2) {
            position2 = index + 1;
        }
    }

    Ok((
        format!("{}", count),
        format!("{}", position1 * position2),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d13_01() {
        let input = String::from("[1,[2,[3,[4,[5,6,7]]]],8,9]\n");
        let mut iter = str_to_buf_reader(&input).bytes();
        let mut result: Vec<Item> = Vec::new();
        process(&mut iter, &mut result).expect("Run failed");
        assert_eq!(
            result,
            [
                Item::Array { items: vec![
                    Item::Number { item: 1 },
                    Item::Array { items: vec![
                        Item::Number { item: 2 },
                        Item::Array { items: vec![
                            Item::Number { item: 3 },
                            Item::Array { items: vec![
                                Item::Number { item: 4 },
                                Item::Array { items: vec![
                                    Item::Number { item: 5 },
                                    Item::Number { item: 6 },
                                    Item::Number { item: 7 },
                                ]},
                            ]},
                        ]},
                    ]},
                    Item::Number { item: 8 },
                    Item::Number { item: 9 }]
                }
            ]
        );

    }

    #[test]
    fn test_d13_02() {
        let input = String::from("[1,1,3,1,1]\n[1,1,5,1,1]\n");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "1",
        );
    }

    #[test]
    fn test_d13_03() {
        let input = String::from("[1,1,5,1,1]\n[1,1,1,1,1]\n");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "0",
        );
    }


    #[test]
    fn test_d13_04() {
        let input = String::from("[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
");
        let result = run(str_to_buf_reader(&input)).expect("Run failed");
        assert_eq!(
            result.0,
            "13",
        );
        assert_eq!(
            result.1,
            "140",
        );
    }

    #[test]
    fn test_d13_final() {
        let f = std::fs::File::open("src/d13/input.txt").expect("No src/13/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "5013",
        );
        assert_eq!(
            result.1,
            "25038",
        );
    }
}
