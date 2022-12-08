use std::{io::{BufReader, BufRead, Read}, collections::HashSet};

const PRIO: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn char_to_priority(ch: char) -> Result<u8, String> {
    match PRIO.find(ch) {
        Some(r) => return Ok((r as u8) + 1),
        None => return Err(format!("Invalid character {}", ch)),
    }
}

fn find_common(s1: &str, s2: &str) -> Option<char> {
    for ch1 in s1.chars() {
        for ch2 in s2.chars() {
            if ch1 == ch2 {
                return Some(ch1);
            }
        }
    }
    return None;
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut total = 0u32;
    let mut total_grouped = 0u32;
    let mut group_index = 0u8;
    let mut common_in_group = HashSet::new();
    for line in input.lines() {
        match line {
            Ok(text) => {
                let half_size = text.len() / 2;
                let first = &text[..half_size];
                let second = &text[half_size..];
                let common = find_common(first, second);
                match common {
                    Some(c) => {
                        match char_to_priority(c) {
                            Ok(prio) => total += prio as u32,
                            Err(e) => return Err(e)
                        }
                    },
                    None => return Err(format!("{} and {} has nothing in common", first, second)),
                }

                if group_index == 0 {
                    common_in_group = HashSet::from_iter(text.chars());
                } else {
                    let text_set = HashSet::from_iter(text.chars());
                    common_in_group = &common_in_group & &text_set; // intersection
                }
                if group_index == 2 {
                    match common_in_group.len() {
                        1 => {
                            match common_in_group.drain().nth(0) {
                                Some(x) => {
                                    match char_to_priority(x) {
                                        Ok(prio) => total_grouped += prio as u32,
                                        Err(e) => return Err(e)
                                    }
                                },
                                None => return Err(format!("Nothing in the group"))
                            }
                            //match char_to_priority(common_in_group.)total_grouped +=
                        },
                        x => return Err(format!("Invalid lenth {} of commons in group: {:?}", x, common_in_group)),
                    }
                    common_in_group.clear();
                    group_index = 0;
                } else {
                    group_index += 1;
                }
            },
            Err(e) => return Err(e.to_string())
        }
    }

    return Ok((total.to_string(), total_grouped.to_string()));
}