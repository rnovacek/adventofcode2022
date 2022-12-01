use std::{env, path::Path};
mod d01a;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: String;
    let input: &Path;
    let filename: String;

    match args.len() {
        2 => {
            day = args[1].clone();
            filename = format!("src/d{}/input.txt", day);
            input = Path::new(&filename);
        },
        3.. => {
            day = args[1].clone();
            input = Path::new(&args[2]);
        },
        _ => {
            panic!("Usage: {} day", args[0]);
        }
    };

    match day.as_str() {
        "01a" => {
            match d01a::run(&input) {
                Ok(()) => {},
                Err(e) => panic!("Failed: {}", e),
            }
        },
        _ => {
            panic!("Unknown day: {}", day);
        }
    }

}
