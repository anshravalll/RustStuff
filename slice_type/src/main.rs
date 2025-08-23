// The problem:

// write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.use std::io;
// use std::io;
// fn first_word(_str: &String) -> Vec<&str> {
//     let arr: Vec<&str> = _str.split(" ").collect();
//     arr
// }

// fn main() {
//     let mut buff = String::new();
//     io::stdin()
//         .read_line(&mut buff)
//         .expect("Error parsing the line");
//     println!("{buff}");
//     let arr = first_word(&buff);
//     let mut parity = true;
//     for element in arr {
//         if element != "" {
//             println!("{:?}", element);
//             parity = false;
//             break; //We can't unfortuantely break out of this with a break return statement because its not loop statement (rust learnings)
//         }
//     }
//     if parity {
//         println!("No word");
//     }
// }

//Based on rust book
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

//Below is the slicing solution
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    char s = "example string";
    first_word(s);
}
