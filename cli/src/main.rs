use std::{env, fs};

#[cfg(test)]
mod tests;

pub struct CountResult {
    words: usize,
    lines: usize,
    chars: usize,
}

pub fn count_by_filepath(path: &str) -> CountResult {
    let read_res = fs::read_to_string(path);
    let content = match read_res {
        Ok(text) => text,
        Err(error) => panic!("{}", error),
    };

    let words: Vec<&str> = content.split_whitespace().collect();
    let lines: Vec<&str> = content.lines().collect();
    let chars: Vec<char> = content.chars().filter(|c| !c.is_whitespace()).collect();

    CountResult {
        words: words.len(),
        lines: lines.len(),
        chars: chars.len(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(path) => path,
        None => panic!("Provide file path: cargo run <path_to_file>"),
    };

    let res = count_by_filepath(path);
    println!("Words: {}", res.words);
    println!("Lines: {}", res.lines);
    println!("Characters: {}", res.chars);
}
