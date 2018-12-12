extern crate regex;

use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Rect {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn parse_rect(data: &str) -> Rect {
    let re = Regex::new(r"#(\d+)\s+.*@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();

    match re.captures(data) {
        Some(caps) => Rect {
            id: caps[1].parse().unwrap(),
            left: caps[2].parse().unwrap(),
            top: caps[3].parse().unwrap(),
            width: caps[4].parse().unwrap(),
            height: caps[5].parse().unwrap(),
        },
        None => panic!(),
    }
}

impl Rect {
    fn intersect(&self, other: &Rect) -> Option<Rect> {
        if self.left + self.width <= other.left {
            return None;
        }

        if other.left + other.width <= self.left {
            return None;
        }

        if self.top + self.height <= other.top {
            return None;
        }

        if other.top + other.height <= self.top {
            return None;
        }

        let left = cmp::max(self.left, other.left);
        let top = cmp::max(self.top, other.top);
        let width = cmp::min(
            self.left + self.width - left,
            other.left + other.width - left,
        );
        let height = cmp::min(self.top + self.height - top, other.top + other.height - top);

        Some(Rect {
            id: 0,
            left,
            top,
            width,
            height,
        })
    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("src/day_3.data")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut rects = Vec::new();

    for line in buffer.lines() {
        let rect = parse_rect(&line);
        rects.push(rect);
    }

    loop {
        let rect = rects.pop().unwrap();

        let len = rects.len();

        if len == 0 {
            println!("Result: {}", rect.id);
            break;
        }

        rects.retain(|ref r| rect.intersect(r).is_none());

        if rects.len() == len {
            println!("Result: {}", rect.id);
            break;
        }
    }

    // 'outer: for r1 in &rects {

    //     for r2 in &rects {
    //         if r1.id == r2.id {
    //             continue;
    //         }

    //         if r1.intersect(&r2).is_some() {
    //             continue 'outer;
    //         }
    //     }

    //     println!("Result: {}", r1.id);
    //     return Ok(());
    // }

    // println!("Nothing found");

    return Ok(());
}
