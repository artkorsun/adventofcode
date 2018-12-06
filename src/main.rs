use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f = File::open("src/test.data")?;
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

        println!("Inspected string is '{}'", first_line );

        'outer: for s in temp_iterator {

            println!("  Processed string is '{}'", s );

            let mut res = String::new();
            let mut mismatch_found = false;
            for c in first_line.char_indices() {
                let i = c.0;

                let char_in_str = s[i..i+1].chars().next();

                if char_in_str.is_some() {
                    println!("      Char by index {} in inspected string '{}', in processed string '{}'", c.0, c.1, char_in_str.unwrap() );
                }
                else
                {
                    println!("      Char is not found in processed string by index {} in inspected string '{}'", c.0, c.1 );
                }

                if c.1 == char_in_str.unwrap() {
                    res.push(c.1);
                    println!("          Chars '{}' and '{}' mathced, current res is {}, origin string '{}', processed string '{}'", c.1, char_in_str.unwrap(), res, first_line, s );
                } else {
                    if mismatch_found {
                        println!("          Second mismatch for chars '{}' and '{}' found, go to outer loop, current res is {}, origin string '{}', processed string '{}'", c.1, char_in_str.unwrap(), res, first_line, s );
                        continue 'outer;
                    } else {
                        println!("          First mismatch for chars '{}' and '{}' found, continue processing, current res is {}, origin string '{}', processed string '{}'", c.1, char_in_str.unwrap(), res, first_line, s );
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
