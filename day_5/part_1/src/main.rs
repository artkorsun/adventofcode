fn main() {
    let input = include_str!("input.data");
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

    print!("{}", res.len());
}
