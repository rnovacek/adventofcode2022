use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::path::Path;

pub fn run(input: &Path) -> Result<(), String> {
    let file = match File::open(input) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string())
    };
    let reader = BufReader::new(file);

    let mut current_sum = 0;
    let mut total_max: i32 = 0;
    for line in reader.lines() {
        match line {
            Ok(text) => {
                println!("{}", text);
                match text.len() {
                    0 => {
                        if current_sum > total_max {
                            total_max = current_sum;
                        }
                        current_sum = 0;
                    },
                    _ => {
                        let num = text.parse::<i32>().unwrap();
                        current_sum += num;
                    }
                }
            },
            Err(e) => return Err(e.to_string())
        }
    }

    println!("Result part one: {}", total_max);

    Ok(())
}
