use std::io;

fn fact(num: u64){
    if num == 0{
        println!("So your factorial answer is 1");
    }

    let mut res:u64 = 1;
    for i in 1..num+1{
        res*=i;
    }
    println!("So your factorial answer is {res}");
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Can't able to read the line");

    let s_to_num: u64 = s.trim().parse().expect("conversion to integer failed");

    fact(s_to_num);
    println!("Hello, world!");
}
