use std::io;

fn first_word(str: &String) -> String {}

fn main() {
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("Error parsing the line");
    println!("{first_word(&buff)}");
}
