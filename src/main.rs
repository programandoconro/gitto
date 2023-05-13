use aho_corasick::AhoCorasick;
use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    println!("Init");
    let input = read_user_input();
    let concatenated_input = clean(concatenate_input(input));
    let command = git_checkout(concatenated_input.clone());
    println!("{}", command);
    let mut input_to_confirm = String::new();
    io::stdin().read_line(&mut input_to_confirm).unwrap();

    let modified_command = command.trim();
    let output = Command::new("sh")
        .arg("-c")
        .arg(modified_command)
        .output()
        .expect("failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprint!("{}", stderr);
    }
    Ok(())
}

fn git_checkout(branch_name: String) -> String {
    "git checkout -b ".to_string() + &branch_name
}

fn read_user_input() -> Vec<String> {
    io::stdin()
        .lines()
        .map(|e| e.unwrap_or("".to_string()))
        .collect()
}

fn concatenate_input(input: Vec<String>) -> String {
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

fn clean(haystack: String) -> String {
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
    let concatenated = concatenate_input(input);
    assert_eq!(concatenated, "hola-chao".to_string());
}

#[test]
fn it_cleans_forbidden_chars() {
    let input = "hola!@#$%^&*()+=?><chao";
    let cleaned = clean(input.to_string());
    assert_eq!(cleaned, "hola-chao".to_string());
}
#[test]
fn it_returns_checkout_command() {
    let command = git_checkout("my-new-branch".to_string());
    assert_eq!(command, "git checkout -b my-new-branch".to_string());
}
