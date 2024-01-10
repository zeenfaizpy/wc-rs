use std::env;
use std::fs;

struct Count {
    lines: usize,
    words: usize,
    chars: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let result = fs::read_to_string(filename);
    let contents = match result {
        Ok(data) => data,
        Err(error) => panic!("Error opening the file {filename} - {error}"),
    };

    let count =  Count {
        lines: contents.lines().count(),
        words: contents.split_whitespace().count(),
        chars: contents.chars().count()
    };

    println!("{} {} {} {filename}", count.lines, count.words, count.chars);
}
