use std::process::Command;

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

#[test]
fn it_executes_shell_commands() {
    let result = execute(format!("echo hola"));
    assert_eq!(result.unwrap(), 0);
}
