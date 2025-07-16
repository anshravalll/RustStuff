//Therer are 4 logical questions that we will be asking to understand ownership and borrowing and related
//  concepts innnnn depth:
//
//  1. Is variable, accessible even after being used as a parameter in a function, in the main
//     function?
//  2. Are we able to modify the variable in the external function?
//  3. Is modification reflected outside the scope? in the main function?
//  4. Is it accessible inside the new function? Yes, always, in all of the below scenarios, it is.
//     Because the function is having the accessibility of the variable.








//Heap based 
    //With ownership
    //With mut
fn foo(mut s: String) -> String{
    println!("Bruh, thisi s your thing? {s}");
    s.push_str("Bye, world!!");
    s
} //This function: Accesses, and returns the variable but doesn't modify it. 

fn main(){
    let mut s = String::from("Hello, World!!"); //Heap based allocation
    let foo_returns = foo(mut s); //There is no reference (borrowing), just pure ownership transfer
    println!("{s}"); //s was moved so its not accessible
    println!("Same s that foo returns is: {foo_returns}");
}
// 1. No, because the variable is MOVED and ownership of the pointer has been changed.
// 2. No, because we are not passing the variable as a mutable reference
// 3. Well, no, because the answer of 1. is No

    
    //With ownership
    //Immutable
// fn foo(s: &String) -> String{
//     println!("Bruh, this i s your thing {s}");
//     s.push_str("Bye, World!!");
//     s
// }
//
// fn main(){
//     let s = String::from("Hello, World!!"); //Heap based allocation
//     foo_returns = foo(s); //Pure ownership transfer without any reference
//     println!("{s}");
//     println!("Same s that foo returns is: {foo_returns}");
// }
