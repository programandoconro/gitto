use std::io;
use aho_corasick::AhoCorasick;


fn main() -> io::Result<()>{
    println!("Init");
    let input = concatenate_input();
    let clean_input = clean(input);
    println!("{}", clean_input);

        Ok(())
}

fn concatenate_input() -> String {

    let mut input = String::new();
    let reader = io::stdin().lines();
    for (i, line) in  reader.enumerate() {
        let new_line = str::replace(&line.unwrap(), " ", "-");
        if i == 0 {
           input = new_line.clone();
        }
        else{
           input = input + &"-".to_string() +  &new_line;
        }

    }
        input
}

fn clean(haystack: String) -> String {
    let patterns = &["@", "/"];
    let replace_with = &["", "-"];

    let ac = AhoCorasick::new(patterns);
    let result = ac.unwrap().replace_all(&haystack, replace_with);

    result
    

}
