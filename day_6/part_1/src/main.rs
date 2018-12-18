extern crate regex;

use regex::Regex;
use std::cmp;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
} 

fn parse_point(record: &str) -> Point {
    
    let re = Regex::new(r"(\d+),\s+(\d+)").unwrap();

    match re.captures(record) {
        Some(caps) => Point {
                                x: caps[1].parse().unwrap(),
                                y: caps[2].parse().unwrap(),
                            },
        None => panic!(),
    }
}

fn calc_distance(x0: i32, x1: i32, y0: i32, y1: i32) -> usize {
    
    (x0 - x1).abs() as usize + (y0 - y1).abs() as usize
}

fn main() {
    let input = include_str!("input.data");
    
    let mut points = Vec::with_capacity(input.len());

    let mut left_top = Point { x: std::usize::MAX,
                               y: std::usize::MAX};
    let mut bottom_right = Point { x: 0,
                                   y: 0};

    for line in input.lines() {
        let point = parse_point(line);

        left_top.x = cmp::min(left_top.x, point.x);
        left_top.y = cmp::min(left_top.y, point.y);
        bottom_right.x = cmp::max(bottom_right.x, point.x);
        bottom_right.y = cmp::max(bottom_right.y, point.y);

        points.push(point);
    }

    let mut matrix = vec![vec![None;bottom_right.y - left_top.y + 1];bottom_right.x - left_top.x + 1];
    
    for x in 0..bottom_right.x - left_top.x + 1{
        for y in 0..bottom_right.y - left_top.y + 1{
            let mut total_distance = 0;

            for p in &points {            
                let dist = calc_distance(x as i32, (p.x - left_top.x) as i32, y as i32, (p.y - left_top.y) as i32);
                total_distance += dist;
            }

            matrix[x][y] = Some(total_distance);
        }
    }

    let mut count = 0;

    for (x, col) in matrix.iter().enumerate() {
        for (y, _row) in col.iter().enumerate() {

            match matrix[x][y] {
                Some(distance) => {
                    if distance < 10000 {
                        count += 1;
                    }
                },
                _ => continue
            }
        }
    }

    print!("{:?}", count);
}
