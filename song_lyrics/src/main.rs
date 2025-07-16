use std::io;
fn print_song() {
    let mut s = String::new();
    let mut v: Vec<String> = Vec::new();
    let mut i = 11;
    println!("Dudey ou gotta write the song now my buoy");
    while !(i == 0) {
        io::stdin().read_line(&mut s).expect("something");
        println!("Thats fucking {}", i);
        v.push(s);
        i -= 1;
    }
    for i in &v {
        print!("{}\n", i);
    }
}

fn main() {
    print_song();
}
