extern crate regex;

use regex::Regex;
use std::cmp;
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

    let mut intersects = Vec::new();

    let mut left = std::u32::MAX;
    let mut right = 0;
    let mut top = std::u32::MAX;
    let mut bottom = 0;

    let mut rect_iterator = rects.iter();
    let mut temp_iterator;

    loop {
        let rect_1 = rect_iterator.next();

        if rect_1.is_none() {
            break;
        }

        temp_iterator = rect_iterator.clone();

        let rect_1 = rect_1.unwrap();

        for rect_2 in temp_iterator {
            match rect_1.intersect(rect_2) {
                Some(r) => {
                    left = cmp::min(left, r.left);
                    right = cmp::max(right, r.left + r.width);
                    top = cmp::min(top, r.top);
                    bottom = cmp::max(bottom, r.top + r.height);

                    intersects.push(r)
                }
                None => continue,
            }
        }
    }

    let mut matrix = vec![vec![0u8; (bottom - top) as usize]; (right - left) as usize];

    for r in intersects {
        let relative_left = r.left - left;
        let relative_top = r.top - top;

        for x in relative_left..relative_left + r.width {
            for y in relative_top..relative_top + r.height {
                matrix[x as usize][y as usize] = 1;
            }
        }
    }

    let mut count = 0;
    for i in matrix {
        for j in i {
            if j == 1 {
                count += 1;
            }
        }
    }

    println!("res: {:?}", count);

    return Ok(());
}
