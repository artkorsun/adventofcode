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
        let split = buffer.split("\n");

        for s in split {
            match s.parse::<i32>() {
                Ok(n) => {
                    res = res + n;

                    if vals.contains( &res ) {
                        println!("{}", res);
                        return Ok(());
                    }
                    else {
                        vals.insert( res );
                    }
                }
                Err(e) => panic!(e),
            }
        }
    }
}