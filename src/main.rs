use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f = File::open("src/day_2.data")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut lines_iterator = buffer.lines();
    let mut temp_iterator;


    loop {

        let first_line = lines_iterator.next();

        if first_line.is_none() {
            println!("Nothing found");
            return Ok(());
        }

        temp_iterator = lines_iterator.clone();

        let first_line = first_line.unwrap();

        'outer: for s in temp_iterator {
            let mut res = String::new();
            let mut mismatch_found = false;
            for c in first_line.char_indices() {
                let i = c.0;

                let char_in_str = s[i..i+1].chars().next();

                if c.1 == char_in_str.unwrap() {
                    res.push(c.1);
                } else {
                    if mismatch_found {
                        continue 'outer;
                    } else {
                        mismatch_found = true;
                        continue;
                    }
                }
            }

            println!("\n\n BINGO! result is '{}'", res);
            return Ok(());
        }
    }
}
