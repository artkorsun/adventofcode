extern crate chrono;
extern crate regex;

use chrono::{NaiveDateTime, Timelike};

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

#[derive(Debug)]
struct Record {
    datetime: NaiveDateTime,
    content: String,
}

#[derive(Debug)]
struct Range {
    from: u32,
    to: u32,
}

fn parse_record(data: &str) -> Record {
    let re = Regex::new(r"\[(.*)\] (.*)").unwrap();

    match re.captures(data) {
        Some(caps) => Record {
            datetime: NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M").unwrap(),
            content: caps[2].to_string(),
        },
        None => panic!(),
    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("src/day_4.data")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut records = Vec::new();

    for line in buffer.lines() {
        let record = parse_record(&line);
        records.push(record);
    }

    records.sort_by(|a, b| a.datetime.cmp(&b.datetime));

    let mut current_guard_id: Option<u32> = None;
    let mut fall_asleep_time: Option<NaiveDateTime> = None;

    let mut guard_sleeps = HashMap::new();
    let guard_id_re = Regex::new(r".*#(\d+).*").unwrap();

    for r in records {
        if r.content == "falls asleep" {
            if current_guard_id.is_none() {
                println!("Unknown guard id");
                panic!();
            }

            fall_asleep_time = Some(r.datetime);
        } else if r.content == "wakes up" {
            if current_guard_id.is_none() {
                println!("Unknown guard id");
                panic!();
            }

            if fall_asleep_time.is_none() {
                println!("No fall asleep event before waking up");
                panic!();
            }

            let guard_data = guard_sleeps
                .entry(current_guard_id.unwrap())
                .or_insert(Vec::new());

            guard_data.push(Range {
                from: fall_asleep_time.unwrap().minute(),
                to: r.datetime.minute(),
            });
            fall_asleep_time = None;
        } else if r.content.starts_with("Guard") {
            match guard_id_re.captures(&r.content) {
                Some(caps) => current_guard_id = Some(caps[1].parse().unwrap()),
                None => panic!(),
            }
        }
    }

    let mut guard_id_with_most_sleeps = 0;
    let mut most_sleeps = 0;

    for (guard_id, sleeps) in &guard_sleeps {
        let mut cur_sleeps = 0;
        for sleep in sleeps {
            cur_sleeps += sleep.to - sleep.from;
        }
        if cur_sleeps > most_sleeps {
            guard_id_with_most_sleeps = *guard_id;
            most_sleeps = cur_sleeps;
        }
    }

    let mut sleep_per_minute = vec![0; 60];
    let sleeps = guard_sleeps.get(&guard_id_with_most_sleeps).unwrap();

    for s in sleeps {
        for m in s.from as usize..s.to as usize {
            sleep_per_minute[m] += 1;
        }
    }

    let mut max_minute = 0;
    let mut minute_idx = 0;
    for (i, m) in sleep_per_minute.iter().enumerate() {
        if m > &max_minute {
            minute_idx = i;
            max_minute = *m;
        }
    }

    println!(
        "\nGuard_id: {:?}\nMinute: {:?}\n\nResult: {}",
        guard_id_with_most_sleeps,
        minute_idx,
        guard_id_with_most_sleeps * minute_idx as u32
    );

    return Ok(());
}
