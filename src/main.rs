use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f = File::open("src/test.data")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    loop {
        let mut lines = buffer.lines();
        let first_line = lines.next();

        if first_line.is_none() {
            println!("Nothing found");
            return Ok(());
        }

        let first_line = first_line.unwrap();

        'outer: for s in lines {
            let mut res = String::new();
            let mut mismatch_found = false;
            for c in first_line.char_indices() {
                let i = c.0;

                if c.1 == s[i..i].chars().next().unwrap() {
                    res.push(c.1);
                } else {
                    if mismatch_found {
                        continue 'outer;
                    } else {
                        mismatch_found = true;
                    }
                }

                println!("{}", res);
                return Ok(());
            }
        }
    }
}
