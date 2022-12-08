use std::{io::{BufReader, BufRead, Read}};

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut fully_contains_count = 0;
    let mut overlap_count = 0;
    for line in input.lines() {
        match line {
            Ok(text) => {
                let parts: Vec<u32> = text.split(&['-', ',']).map(|num| num.parse::<u32>().unwrap()).collect();
                if parts[0] >= parts[2] && parts[1] <= parts[3] {
                    fully_contains_count += 1;
                    overlap_count += 1;
                    // println!("First fully contained in second: {}-{},{}-{}", parts[0], parts[1], parts[2], parts[3])
                } else if parts[0] <= parts[2] && parts[1] >= parts[3] {
                    fully_contains_count += 1;
                    overlap_count += 1;
                    // println!("Second fully contained in first: {}-{},{}-{}", parts[0], parts[1], parts[2], parts[3])
                } else if parts[0] <= parts[2] && parts[1] >= parts[2] {
                    overlap_count += 1;
                    // println!("First overlaps with second: {}-{},{}-{}", parts[0], parts[1], parts[2], parts[3])
                } else if parts[0] <= parts[3] && parts[1] >= parts[3] {
                    overlap_count += 1;
                    // println!("Second overlaps with first: {}-{},{}-{}", parts[0], parts[1], parts[2], parts[3])
                }
            },
            Err(e) => return Err(e.to_string())
        }
    }

    return Ok((fully_contains_count.to_string(), overlap_count.to_string()));
}
