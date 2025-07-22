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
//With mut
//With ownership
// fn foo(mut s: String) -> String{
//     println!("Bruh, thisi s your thing? {s}");
//     s.push_str("Bye, world!!");
//     s
// } //This function: Accesses, and returns the variable but doesn't modify it.
//
// fn main(){
//     let mut s = String::from("Hello, World!!"); //Heap based allocation
//     let foo_returns = foo(mut s); //There is no reference (borrowing), just pure ownership transfer
//     println!("{s}"); //s was moved so its not accessible
//     println!("Same s that foo returns is: {foo_returns}");
// }
// 1. No, because the variable is MOVED and ownership of the pointer has been changed.
// 2. No, because we are not passing the variable as a mutable reference
// 3. Well, no, because the answer of 1. is No

// MISPLACED
//With ownership
//Immutable
// fn foo(s: &String) -> String{ //& doesn't work here, as it is purely for reference (borrowing)
// purposes.
//
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

//With borrowing
//with reference

// fn foo(s: &String) {
//     let mut s = String::from("Unexpected");
//     println!("Nott trying to modify this thing rn == {s}");
// }
//
// fn main(){
//     let mut s = String::from("Hello, World!!");
//     foo(&s);
//     s = String::from("Completely different thing ");
//
//     println!("this is your s: {s}");
// }

// 1. Yes, because there is no transfer of ownership just references
// 2. No, the condition to allow variable to change data for heap based data is to have it mutable and then to have its mutable reference rather than the immutable one.
// (maybe above all answers are sufficient)

//With borrowing
//with mut
//with reference

fn foo(s: &mut String) -> &String {
    println!("Hey this is raw s from foo function, s: {s}");
    s = String::from("Bye, World!!!");
    s
}
fn main() {
    let mut x = String::from("Hello, World!!");
    let foo_returns = foo(&mut x);

    println!("This is x: {}", x);
    println!("This is foo_returns: {}", foo_returns);
}

//something needs to be understood about this snippet, as i can't be able to do ln 97 x printing,
//it is suggesting me to not do an "immutable borrow" because i am already doing "mutable borrow"
//at line 95
