use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut f = File::open("src/day_2.data")?;
    //let mut f = File::open("src/test.data")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut with_2 = 0;
    let mut with_3 = 0;

    for s in buffer.lines() {
        let mut vals = HashMap::new();
        for c in s.chars() {
            
            let val = vals.entry(c).or_insert( 0 );
            *val += 1;
        }

        let mut _2 = false;
        let mut _3 = false;

        for v in vals.values() {
            _2 = _2 || *v == 2;
            _3 = _3 || *v == 3;
        }

        if _2 {
            with_2 += 1;
        }

        if _3 {
            with_3 += 1;
        }
    }

    println!("{}", with_2 * with_3);
    return Ok(());
}