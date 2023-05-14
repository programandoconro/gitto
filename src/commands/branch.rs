use aho_corasick::AhoCorasick;
use super::utils::{execute, confirm, command_output};
extern crate skim;
use skim::prelude::*;
use std::io::Cursor;

pub fn switch() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let input = command_output("git", vec!["branch", "-a"]);

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
        let command = "git checkout".to_string() + &item.output();
        println!("{}", command);
        if confirm().is_ok() {
           execute(command).expect("There was an error executing git command");
        }
    }
}



pub fn create() {
    let input = read_user_input();
    let concatenated_input = sanitize(concatenate(input));
    let command = prepend_gitcheckout(concatenated_input);
    println!("{}", command);

    if confirm().is_ok() {
       execute(command).expect("There was an error executing git command");
    }
}



fn prepend_gitcheckout(branch_name: String) -> String {
    "git checkout -b ".to_string() + &branch_name
}

fn read_user_input() -> Vec<String> {
    println!("Enter branch name, then press Enter and Ctrl-D to end:");

    std::io::stdin()
        .lines()
        .map(|e| e.unwrap_or("".to_string()))
        .collect()
}

fn concatenate(input: Vec<String>) -> String {
    let mut concatenated: String = "".to_string();
    for line in input {
        if concatenated.len() == 0 {
            concatenated = line;
        } else {
            let new_line = str::replace(&line, " ", "-");
            concatenated = concatenated + &"-".to_string() + &new_line;
        }
    }
    concatenated
}

fn sanitize(haystack: String) -> String {
    let patterns = &[
        "@", "/", " ", "^", "~", ":", "*", "?", "[", "]", "#", "$", "%", "&", "+", "=", "(", ")",
        "!", "'", "\"", "#", "+", ">", "<", "?",
    ];
    let replace_with = &["-"].repeat(patterns.len());
    let ac = AhoCorasick::new(patterns);
    let result = ac.unwrap().replace_all(&haystack, replace_with);
    let mut clean: String = String::new();

    for (i, current_char) in result.chars().enumerate() {
        let next_char = result.chars().nth(i + 1).unwrap_or('-');
        if current_char == '-' && next_char == '-' || clean.len() == 0 && current_char == '-' {
            continue;
        }
        clean.push(current_char);
    }

    clean
}

#[test]
fn it_handles_line_breaks() {
    let input = vec!["hola".to_string(), "chao".to_string()];
    let concatenated = concatenate(input);
    assert_eq!(concatenated, "hola-chao".to_string());
}

#[test]
fn it_sanitizes_forbidden_chars() {
    let input = "hola!@#$%^&*()+=?><chao";
    let cleaned = sanitize(input.to_string());
assert_eq!(cleaned, "hola-chao".to_string());
}

#[test]
fn it_returns_checkout_command() {
    let command = prepend_gitcheckout("my-new-branch".to_string());
    assert_eq!(command, "git checkout -b my-new-branch".to_string());
}

