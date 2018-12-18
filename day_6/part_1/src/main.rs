extern crate regex;

use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

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
    
    'col: for x in 0..bottom_right.x - left_top.x + 1{
        'row: for y in 0..bottom_right.y - left_top.y + 1{
            let mut point_per_distance = HashMap::new();

            for (i, p) in points.iter().enumerate() {
                
                if x == p.x - left_top.x && y == p.y - left_top.y {
                    matrix[x][y] = Some(i);
                    continue 'row;
                }

                let dist = calc_distance(x as i32, (p.x - left_top.x) as i32, y as i32, (p.y - left_top.y) as i32);
                let points = point_per_distance.entry(dist).or_insert(vec![]);
                points.push(i);
            }

            let min_dist = point_per_distance.keys().min().unwrap();
            let points_with_mit_distance = point_per_distance.get( min_dist ).unwrap();

            if points_with_mit_distance.len() == 1 {
                matrix[x][y] = Some(points_with_mit_distance[0]);
            }
        }
    }

    let mut area_size_per_point = HashMap::new();
    let mut points_with_infinite_area = HashSet::new();

    for (x, col) in matrix.iter().enumerate() {
        for (y, _row) in col.iter().enumerate() {

            match matrix[x][y] {
                Some(index) => {
                let vals = area_size_per_point.entry(index).or_insert(0);
                    *vals += 1;

                    if x == 0 || y == 0 || x == matrix.len() - 1 || y == col.len() - 1 {
                        points_with_infinite_area.insert(index);
                    }   
                },
                _ => continue
            }
        }
    }

    let mut max_area = 0;
    for (i, area_size) in &area_size_per_point {
        
        if !points_with_infinite_area.contains(i) {
            max_area = cmp::max(max_area, *area_size);
        }
    }

    print!("{:?}\n", max_area);
}
