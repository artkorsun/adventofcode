use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut f = File::open("src/day_1.data")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut vals = HashSet::new();

    let mut res = 0;

    loop {

        for s in buffer.lines() {
            let n:i32 = s.parse().unwrap();

            res = res + n;

            if !vals.insert( res ) {
                println!("{}", res);
                return Ok(());
            }
        }
    }
}