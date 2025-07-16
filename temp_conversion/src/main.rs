use std::io;

fn convert_to_ferehnhit(something: u32){
    let result = (something * 9 / 5) + 32;
    println!("Ferenhit {}", result)
}

fn main() {
    let _num = 49;
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("something");
    let num: u32 = num.trim().parse().expect("failed bruh");
    convert_to_ferehnhit(num);
} 
