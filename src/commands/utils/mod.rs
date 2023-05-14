use std::process::Command;
use std::io::{self, BufRead, Error};

pub fn command_output(command: &str, args: Vec<&str>) -> String {
     let output = Command::new(command)
        .args(&args)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    let mut result = String::new();
    for branch in stdout.lines() {
        result = result + "\n" + branch;
    }

    result
}

pub fn execute(command: String) -> Result<i32, i32> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
        Ok(0)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprint!("{}", stderr);
        Err(1)
    }
}

pub fn confirm() -> Result<bool, Error> {
    let mut input = String::new();
    let stdin = io::stdin();

    ctrlc::set_handler(move || {
        println!("Bye...");
        std::process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Press ENTER to confirm or Ctrl-C to cancel");

    match stdin.lock().read_line(&mut input) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}


#[test]
fn it_executes_shell_commands() {
    let result = execute(format!("echo hola"));
    assert_eq!(result.unwrap(), 0);
}
