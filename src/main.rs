use aho_corasick::AhoCorasick;
use std::io;

fn main() -> io::Result<()> {
    println!("Init");
    let input = concatenate_input();
    let clean_input = clean(input);
    println!("{}", clean_input);

    Ok(())
}

fn concatenate_input() -> String {
    let mut input = String::new();
    let reader = io::stdin().lines();
    for (i, line) in reader.enumerate() {
        let new_line = str::replace(&line.unwrap(), " ", "-");
        if i == 0 {
            input = new_line.clone();
        } else {
            input = input + &"-".to_string() + &new_line;
        }
    }
    input
}

fn clean(haystack: String) -> String {
    let patterns = &[
        "@", "/", " ", "^", "~", ":", "*", "?", "[", "]", "#", "$", "%", "&", "+", "=",
        "(", ")", "!", "'", "\"", "#", "+"
    ];
    let replace_with = &["-"].repeat(patterns.len());
    let ac = AhoCorasick::new(patterns);
    let result = ac.unwrap().replace_all(&haystack, replace_with);
    let mut clean: String = String::new();

    for (i, current_char) in result.chars().enumerate() {
        let next_char = result.chars().nth(i + 1).unwrap_or('-');
        if current_char == '-' && next_char == '-' ||
           clean.len() == 0 && current_char == '-' {
            continue;
        }
            clean.push(current_char);
    }

    clean
}
