use std::{io::{Read, BufReader, BufRead}, ops::Add};

type FileRef = usize;

struct FileSystem {
    curdir_idx: FileRef,
    files: Vec<File>,
}

impl FileSystem {
    fn new() -> Self {
        let root = File::new("/", None, 0);
        Self {
            curdir_idx: 0,
            files: vec![root],
        }
    }

    fn get_or_create(&mut self, name: &String, size: u32) -> FileRef {
        let curdir = &self.files[self.curdir_idx];
        for child_index in curdir.children.iter() {
            let child = &self.files[*child_index];
            if child.name.eq(name) {
                return *child_index;
            }
        }

        let new_file = File::new(name, Some(self.curdir_idx), size);
        let index = self.files.len();
        self.files[self.curdir_idx].children.push(index);
        self.files.push(new_file);

        // Update sizes of all parents
        let mut parent_index = self.curdir_idx;
        loop {
            self.files[parent_index].size += size;
            match self.files[parent_index].parent {
                Some(index) => {
                    parent_index = index
                }
                None => break
            }
        }
        index
    }

    fn execute(&mut self, command: &Command) -> Result<(), String> {
        let curdir = &self.files[self.curdir_idx];

        match command {
            Command::CD { name } => match name.as_str() {
                ".." => match curdir.parent {
                    Some(dir) => {
                        self.curdir_idx = dir;
                        Ok(())
                    },
                    None => Err(format!("Cannot move up from {}", curdir.name)),
                },
                "/" => {
                    self.curdir_idx = 0;
                    Ok(())
                },
                name => {
                    self.curdir_idx = self.get_or_create(&String::from(name), 0);
                    Ok(())
                },
            },
            Command::LS {} => {
                Ok(())
            },
            Command::FILE { name, size } => {
                self.get_or_create(name, *size);
                Ok(())
            },
            Command::DIR { name } => {
                self.get_or_create(name, 0);
                Ok(())
            },
        }
    }


    fn _print(&self) {
        self._print_file(&self.files[0], "");
    }

    fn _print_file(&self, file: &File, indent: &str) {
        file._print(&indent);
        for index in file.children.iter() {
            self._print_file(&self.files[*index], &String::from(indent).add("  "));
        }
    }
}

struct File {
    name: String,
    size: u32,
    isdir: bool,
    parent: Option<FileRef>,
    children: Vec<FileRef>,
}

impl File {
    fn new(name: &str, parent: Option<FileRef>, size: u32) -> Self {
        Self {
            name: String::from(name),
            size,
            children: Vec::new(),
            parent,
            isdir: size == 0,
        }
    }

    fn _print(&self, indent: &str) {
        if self.isdir {
            println!("{}- {} (dir, size={})", indent, self.name, self.size);
        } else {
            println!("{}- {} (file, size={})", indent, self.name, self.size);
        }
    }
}

const SIZE_LIMIT: u32 = 100000u32;
const TOTAL_SPACE: u32 = 70000000u32;
const REQUIRED_SPACE: u32 = 30000000u32;

enum Command {
    CD { name: String },
    LS {},
    DIR { name: String },
    FILE { name: String, size: u32 },
}


fn parse(line: String) -> Result<Command, String> {
    let mut parts = line.split(' ');
    let first = parts.next();
    let second = parts.next();
    let third = parts.next();

    match first {
        Some("$") => match second {
            Some("cd") => match third {
                Some(dir) => Ok(Command::CD {
                    name: String::from(dir),
                }),
                None => Err(String::from("Missing argument for cd")),
            },
            Some("ls") => Ok(Command::LS {}),
            Some(x) => Err(format!("Invalid command {}", x)),
            None => Err(String::from("Unfinished command line"))
        },
        Some("dir") => match second {
            Some(name) => Ok(Command::DIR {
                name: String::from(name),
            }),
            None => Err(String::from("No dir name"))
        },
        Some(number) => {
            match number.parse::<u32>() {
                Ok(size) => match second {
                    Some(name) => Ok(Command::FILE {
                        name: String::from(name),
                        size,
                    }),
                    None => Err(String::from("Filesize without filename"))
                },
                Err(_) => Err(format!("Invalid operation {}", number))
            }
        },
        None => Err(String::from("Empty line")),
    }
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut fs = FileSystem::new();

    for line in input.lines() {
        match line {
            Ok(ln) => match parse(ln) {
                Ok(command) => match fs.execute(&command) {
                    Ok(_) => {},
                    Err(msg) => return Err(msg)

                },
                Err(msg) => return Err(msg)
            },
            Err(_) => return Err(String::from("Cannot read line"))
        }
    }
    // Enable to print FS layout
    // fs._print();

    let mut total_under_limit = 0;
    let used_size = fs.files[0].size;
    let mut smallest_to_delete = used_size;
    for file in fs.files {
        if file.isdir {
            if file.size < SIZE_LIMIT {
                total_under_limit += file.size;
            }

            if TOTAL_SPACE - used_size + file.size > REQUIRED_SPACE && file.size < smallest_to_delete {
                smallest_to_delete = file.size;
            }

        }
    }

    Ok((
        format!("{}", total_under_limit),
        format!("{}", smallest_to_delete),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::str_to_buf_reader;

    #[test]
    fn test_d07_01() {
        let input = String::from("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");
        assert_eq!(
            run(str_to_buf_reader(&input)).expect("Error").0,
            "95437"
        );
    }

    #[test]
    fn test_d07_final() {
        let f = std::fs::File::open("src/d07/input.txt").expect("No src/d07/input.txt file");
        let result = run(BufReader::new(f)).expect("Run failed");
        assert_eq!(
            result.0,
            "1611443",
        );
        assert_eq!(
            result.1,
            "2086088",
        );
    }
}
