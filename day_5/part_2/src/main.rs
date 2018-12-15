use std::cmp;
use std::char;

fn remove_item(input: &str, char_to_remove: char) -> String
{
    let mut res = String::new();

    for c in input.chars() {
        if char_to_remove.to_ascii_uppercase() == c.to_ascii_uppercase() {
            continue;
        }
        else
        {
            res.push(c);
        }
    }
    res
}

fn react_polymer(input: &str) -> String {
    let mut res = String::new();

    for c in input.chars() {
        
        if res.is_empty() {
            res.push(c);
            continue;
        }

        let last_char = res.pop().unwrap();

        if last_char != c && last_char.to_ascii_uppercase() == c.to_ascii_uppercase() {
            continue;
        }
        else
        {
            res.push(last_char);
            res.push(c);
        }
    }
    res
}

fn main() {
    
    let mut min_len = std::usize::MAX;

    for i in 97..123 {
        let c = char::from_u32(i).unwrap();
        
        let input = include_str!("input.data");
        let reduced = remove_item(input, c);
        let res = react_polymer(&reduced);

        min_len = cmp::min(min_len, res.len()); 
    }

    print!("{}", min_len);
}
