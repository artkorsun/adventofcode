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

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Point {
        index: usize,
    },
    Distance {
        index: usize,
        distance: usize,
    },
    Shared {
        distance: usize,
    },
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

fn calc_distance(x0: i8, y0: i8, x1: i8, y1: i8) -> usize {
    
    (x0 - x1).abs() as usize + (y0 - y1).abs() as usize
}

fn main() {
    let input = include_str!("test.data");
    
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

    let mut matrix = vec![vec![Cell::Empty{};bottom_right.y - left_top.y + 1];bottom_right.x - left_top.x + 1];
    
    'col: for x in 0..bottom_right.x - left_top.x + 1{
        'row: for y in 0..bottom_right.y - left_top.y + 1{
            for (i, p) in points.iter().enumerate() {
                
                if x == p.x - left_top.x && y == p.y - left_top.y {
                    matrix[x][y] = Cell::Point{index: i};
                    continue 'row;
                }

// 1, 1
// 1, 6
// 8, 3
// 3, 4
// 5, 5
// 8, 9
                let dist = calc_distance(x as i8, (p.x - left_top.x) as i8, y as i8, (p.y - left_top.y) as i8);

                match matrix[x][y] {
                    Cell::Point{ index: _ } => {
                        continue 'row;
                    },
                    Cell::Distance{ index: _, distance } => {
                        if dist < distance {
                            matrix[x][y] = Cell::Distance {
                                                index: i,
                                                distance: dist,
                                            };    
                        }
                        if dist == distance {
                            matrix[x][y] = Cell::Shared {
                                                distance: dist,
                                            };
                        }
                    },
                    Cell::Shared{ distance } => {
                        if dist < distance {
                            matrix[x][y] = Cell::Distance {
                                                index: i,
                                                distance: dist,
                                            };    
                        }
                    },
                    Cell::Empty => {
                        matrix[x][y] = Cell::Distance {
                                                index: i,
                                                distance: dist,
                                            };
                    },
                    
                    
                }
            }
        }
    }

    print!("matrix: {:?}\n", matrix);

    let mut area_size_per_point = HashMap::new();
    let mut points_with_infinite_area = HashSet::new();

    for (x, col) in matrix.iter().enumerate() {
        for (y, _row) in col.iter().enumerate() {
            
            match matrix[x][y] {
                Cell::Distance{ index, distance: _ } => {
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

    print!("area_size_per_point: {:?}\n", area_size_per_point);
    print!("points_with_infinite_area: {:?}\n", points_with_infinite_area);

    let mut max_area = 0;
    for (i, area_size) in &area_size_per_point {
        
        if !points_with_infinite_area.contains(i) {
            max_area = cmp::max(max_area, *area_size);
        }
    }

    print!("{:?}\n", max_area);
}
