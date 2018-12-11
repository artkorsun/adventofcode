extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::cmp;

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
    fn intersect(&self, other: Rect) -> Option<Rect> {

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

        let left = cmp::max( self.left, other.left );
        let top = cmp::max( self.top, other.top );
        let width = cmp::min( self.left + self.width - left, other.left + other.width - left );
        let height =  cmp::min( self.top + self.height - top, other.top + other.height - top);

        Some( Rect{ left, top, width, height } )
    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("src/test.data")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut lines_iterator = buffer.lines();
    let mut temp_iterator;

    let mut intersects = Vec::new();

    loop {
        let first_line = lines_iterator.next();

        if first_line.is_none() {
            break;
        }

        temp_iterator = lines_iterator.clone();

        let first_line = first_line.unwrap();

        let rect = parse_rect(&first_line);

        println!("Current rect: {:?}", rect);

        for s in temp_iterator {
            let rect_2 = parse_rect(&s);

            match rect.intersect( rect_2 ) {
                Some( r ) => intersects.push( r ),
                None => continue
            }
        }
    }

    for r in intersects {
        println!("  Processed rect: {:?}", r);
    }

    return Ok(());
}
