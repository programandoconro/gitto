use aho_corasick::AhoCorasick;
use std::{io, fmt::Error};

fn main() -> io::Result<()> {
    println!("Init");

    let command = git_checkout();  
    println!("{}", command);
    Ok(())
}

fn git_checkout() -> String {
    let input = read_user_input();
    let concatenated_input = concatenate_input(input);
   clean(concatenated_input)
}

fn read_user_input() -> Vec<String> {
    let reader = io::stdin().lines();
    let mut lines = Vec::new();
    for (i, line) in reader.enumerate() {
        lines.push(line.unwrap());
    }

    lines

}

fn concatenate_input(input:Vec<String>) -> String {
    let mut concatenated: String = "".to_string();
    for line in input{
        if concatenated.len() == 0 {
            concatenated = line;
        }
        else{
        let new_line = str::replace(&line, " ", "-");
        concatenated = concatenated + &"-".to_string() + &new_line;
        }
    }
    concatenated
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


#[test]
fn it_handles_line_breaks(){
    let input = vec!["hola".to_string(), "chao".to_string()];
    let concatenated = concatenate_input(input);
    assert_eq!(concatenated, "hola-chao".to_string());
    
}



