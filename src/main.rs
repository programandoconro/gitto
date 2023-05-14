mod commands;

fn main() {
    println!("Init");

    commands::branch::switch();
//    commands::branch::create();

    println!("Finished");

}
