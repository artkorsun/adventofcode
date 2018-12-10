extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Rect {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn parse_rect(data: &str) -> Rect {
    let re = Regex::new(r".*@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();

    match re.captures(data) {
        Some(caps) => Rect {
            left: caps[1].parse().unwrap(),
            top: caps[2].parse().unwrap(),
            width: caps[3].parse().unwrap(),
            height: caps[4].parse().unwrap(),
        },
        None => panic!(),
    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("src/test.data")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut lines_iterator = buffer.lines();
    let mut temp_iterator;

    loop {
        let first_line = lines_iterator.next();

        if first_line.is_none() {
            println!("Nothing found");
            return Ok(());
        }

        temp_iterator = lines_iterator.clone();

        let first_line = first_line.unwrap();

        let rect = parse_rect(&first_line);

        println!("Current rect: {:?}", rect);

        for s in temp_iterator {
            let rect_to_process = parse_rect(&s);

            println!("  Processed rect: {:?}", rect_to_process);
        }
    }

    return Ok(());
}
