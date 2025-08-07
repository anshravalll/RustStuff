use std::io;

fn first_word(_str: &String) -> Vec<&str> {
    let arr: Vec<&str> = _str.split(" ").collect();
    arr
}

fn main() {
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("Error parsing the line");
    println!("{buff}");
    let arr = first_word(&buff);
    let mut parity = true;
    for element in arr {
        if element != "" {
            println!("{:?}", element);
            parity = false;
            break; //We can't unfortuantely break out of this with a break return statement because its not loop statement
        }
    }
    if parity {
        println!("No word");
    }
}
