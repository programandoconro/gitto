use super::utils::{command_output, confirm, execute};
extern crate skim;
use skim::prelude::*;
use std::io::Cursor;

///
/// It creates a new git branch parsing and cleaning input. You can add multiples lines, incorrect
///  and non ASCII characters, and it will concatenate the name  using "-" as separator.
/// # Examples
/// rusty-git-commands --command create
///
pub fn create() {
    let input = read_user_input();
    let concatenated_input = sanitize(concatenate(input));
    let command = prepend_gitcheckout(concatenated_input);
    println!("{}", command);

    if confirm().is_ok() {
        execute(&command).expect("There was an error executing git command");
    }
}

///
/// It switches to a different branch interactively using fzf. You can search best match, move the
/// cursor or just hit enter to select the target branch.
/// # Examples
/// rusty-git-commands --command switch
///
pub fn switch() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let branches = command_output("git", vec!["branch", "-a"]);

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(branches));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
        let command = "git checkout".to_string() + &item.output();
        println!("{}", command);
        if confirm().is_ok() {
            execute(&command).expect("There was an error executing git command");
        }
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
    let mut name = String::new();
    for ch in haystack.chars() {
        if ch.is_alphanumeric() && ch.is_ascii() {
            name.push(ch);
        } else {
            let last_char = name.chars().last().unwrap_or('-');
            if last_char != '-' {
                name.push('-')
            }
        }
    }
    limit_name_len(name)
}

fn limit_name_len(name: String) -> String {
    if name.len() < 100 {
        name
    } else {
        name[0..99].to_string()
    }
}

#[test]
fn it_limits_name_len() {
    let long_name = "very long name".repeat(100);
    assert!(long_name.len() >= 100);
    assert!(limit_name_len(long_name).len() < 100);
}

#[test]
fn it_handles_line_breaks() {
    let input = vec!["hola".to_string(), "chao".to_string()];
    let concatenated = concatenate(input);
    assert_eq!(concatenated, "hola-chao".to_string());
}

#[test]
fn it_sanitizes_forbidden_chars() {
    let input = "hola!@#$%^&*()+=?><コンに感chao";
    let cleaned = sanitize(input.to_string());
    assert_eq!(cleaned, "hola-chao".to_string());
}

#[test]
fn it_returns_checkout_command() {
    let command = prepend_gitcheckout("my-new-branch".to_string());
    assert_eq!(command, "git checkout -b my-new-branch".to_string());
}
