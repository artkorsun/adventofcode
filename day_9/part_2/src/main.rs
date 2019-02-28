extern crate regex;
extern crate typed_arena;

use std::collections::HashMap;
use regex::Regex;
use std::cell::Cell;
use typed_arena::Arena; 
use std::cmp;

fn parse_input(data: &str) -> (i32, i32) {
    let re = Regex::new(r"(\d*) players; last marble is worth (\d*) points").unwrap();

    match re.captures(data){
        Some(caps) => (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
        None => panic!(),
    }
}

#[derive(Debug)]
struct Marble<'a> {
    value: u64,
    next: Cell<Option<&'a Marble<'a>>>,
    prev: Cell<Option<&'a Marble<'a>>>,
} 

impl<'a> Marble<'a> {
    fn zero(arena: &'a Arena<Marble<'a>>) -> &'a Marble<'a> {
        let marble = arena.alloc(Marble {
            value: 0,
            next: Cell::new(None),
            prev: Cell::new(None),
        });

        marble.prev.set(Some(marble));
        marble.next.set(Some(marble));

        marble
    }

    fn new(value: u64, arena: &'a Arena<Marble<'a>>, current: &'a Marble<'a>) -> &'a Marble<'a> {
        let marble = arena.alloc(Marble {
            value,
            next: Cell::new(None),
            prev: Cell::new(None),
        });

        let insert_after = current.next.get().unwrap();

        insert_after.next.get().unwrap().prev.set(Some(marble));
        marble.next.set(insert_after.next.get());
        insert_after.next.set(Some(marble));
        marble.prev.set(Some(insert_after));

        marble
    }

    fn take_7th_counterclock_wise_and_new_current(current: &'a Marble<'a>) -> (u64, &'a Marble<'a>) {

        let mut seventh = current;

        for _ in 0..7 {
            seventh = seventh.prev.get().unwrap();
        }

        let val = seventh.value;
        let next = seventh.next.get().unwrap();

        seventh.prev.get().unwrap().next.set(seventh.next.get());
        seventh.next.get().unwrap().prev.set(seventh.prev.get());

        (val, next)
    }
}

fn main() {
    
    let input = include_str!("test.data");
    let input = parse_input(input);

    let arena = Arena::new();

    let zero_marble = Marble::zero(&arena);
        
    let mut current = zero_marble;
    
    let mut player = 1;

    let mut score = HashMap::new();

    for i in 1..input.1 + 1 {
        
        if i % 23 == 0 {
            let mut points = i as u64;
            let res = Marble::take_7th_counterclock_wise_and_new_current(current);

            points += res.0;
            current = res.1;
                
            let player_points = score.entry(player).or_insert(0 as u64);
            *player_points += points;

            continue;
        }

        let marble = Marble::new(i as u64, &arena, current);
        current = marble;

        if player == input.0 {
            player = 1;
        } 
        else
        {
            player = player + 1;
        } 
    }

    let mut max_points = 0;
    for (_, points) in &score {
        max_points = cmp::max(max_points, *points);
    }

    println!("Max points: {:?}", max_points);
}