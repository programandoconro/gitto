mod commands;

fn main() {
    println!("Init");

    commands::branch::create();

    println!("{}", "Finished");
}
