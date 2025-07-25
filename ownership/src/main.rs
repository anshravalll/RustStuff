//Therer are 4 logical questions that we will be asking to understand ownership and borrowing and related
//  concepts innnnn depth:
//
//  1. Is the above snippet compilable?
//  2. Is variable, accessible even after being used as a argument in a function, in the main
//     function?
//  3. Are we able to modify the variable in the external function?
//  4. Is modification reflected outside the scope? in the main function?
//  5. Is it accessible inside the new function? Yes, always, in all of the below scenarios, it is.
//     Because the function is having the accessibility of the variable.

//1.
//Heap based
//With Ownership
//(Mutable Variable, Mutable binding in the external function parameter)
// fn foo(mut s: String) -> String {
//     println!("Bruh, thisi s your thing? {s}");
//     s.push_str("Bye, world!!");
//     s
// } //This function: Accesses, and returns the variable but doesn't modify it.

// fn main() {
//     let mut s = String::from("Hello, World!!"); //Heap based allocation
//     let foo_returns = foo(s); //There is no reference (borrowing), just pure ownership transfer
//     // println!("{s}"); //s was moved so its not accessible
//     println!("Same s that foo returns is: {foo_returns}");
// }
// 1. Yes
// 2. No, because the variable is MOVED and ownership of the pointer has been changed.
// 3. No, because we are not passing the variable as a mutable reference
// 4. Well, no, because the answer of 2. is No

// 2.
// Heap Based
//With ownership
//(Nothing mutable at all)
// fn foo(s: String) -> String{ //No mutable binding, no mutable reference
//
//     println!("Bruh, this i s your thing {s}");
//     s.push_str("Bye, World!!");
//     s
// }
//
// fn main(){
//     let s = String::from("Hello, World!!"); //Heap based allocation
//     foo_returns = foo(s); //Pure ownership transfer without any reference
//     //println!("{s}"); //Invalid line, due to pure ownership transfer
//     println!("Same s that foo returns is: {foo_returns}");
// }
// 1. Yes
// 2. No, Because the ownership has been completely transferred to the external function.
// 3. No, because there must either be mutable binding or mutable refeerence in the functional parameters, but here, its just pure immutability.
// 4. No, Due to ownership transfer we can't see the new value, let alone modify it.

//Scenarios to explore in heap based borrowing and the reasoning behind it:
// see, first of all question of borrowing and ownership comes to mind when we are talking about heap and memory.
// So, lets say we have a data stored in a memory and have a name (variable) assigned to it
// regardless of this variables mutability (mutable or not), i can atleast reference it which means borrow it, be it mutable referencing or not.
// The only relevant scenario to explore is to make the initial variable mutable, so that we can get the idea of both mutable reference and immutable reference.
//
// 3.
//Heap based
//With borrowing
//Immutable reference with the capabilities of mutation from initial variable initialization.

// fn foo(s: &String) {
//     println!("This s ='{s}', is just readable, if we try to write ini t, it despises.");
//     // *s = String::from("blah"); //dereference (*) it or directly pass the value to the variable, THERE IS NO MUT SO NO CHANGE AND THIS MOVE IS ILLEGAL.
// }

// fn main() {
//     let mut s = String::from("Hello, World!!"); //Mutable owner
//     let x = &s; //Creating an immutable reader.
//     foo(x); //Using that reader and passing it to the function

//     println!("this is your s: {x}"); //We are able to access the reader, because the reader is valid, why? because the owner is valid, as long as the owner s is valid, reader will be.
// }
// 1. Yes.
// 2. Yes, because we are just dealing with the reader.
// 3. No, the condition to allow variable to change data for heap based data is to have it mutable and then to have its mutable reference rather than the immutable one.
// 4. No, just because of the 3rd reason. We don't have mutable reference.

//4.
//Heap based
//With borrowing
//with reference
//With mutable capabilities (from first variable initialization) and with mutable referencing.

// fn foo(s: &mut String) -> &String {
//     println!("Modifying string inside foo...");
//     // Dereference `s` to change the value it points to (which is `x` from main).
//     // We could've also used s.(method to update the string, if we wanted 'no dereferencing')
//     *s = String::from("Bye, World!!!");
//     // Return the reference. Now it's an immutable reference.
//     s
// }

// fn main() {
//     // 1. `x` is created and owns the "Hello, World!!" data.
//     let mut x = String::from("Hello, World!!");

//     // 2. We call `foo`, passing a MUTABLE borrow of `x`.
//     //    - Inside `foo`, `x`'s value is changed to "Bye, World!!!".
//     //    - `foo` returns an IMMUTABLE borrow to `x`.
//     //    - `foo_returns` now holds this immutable borrow.
//     //    - CRITICAL: Because `foo_returns` exists, `x` is considered immutably borrowed
//     //      until `foo_returns` goes out of scope.
//     let foo_returns = foo(&mut x);

//     // 3. We try to print `foo_returns`.
//     println!("This is foo_returns: {}", foo_returns);

//     // 4. We try to print `x`.
//     println!("This is x: {}", x);
// }
//
// 1. Yes
// 2. Yes, because of mut referencing
// 3. Yes, because of mut referencing
// 4. yes, I am trying to access the value by println! macro. This macro creates temporary immutable reference to the thing getting printed so make sure there is no other mutable reference alive.

//Main memory based

//A try to understand the ownership and borrowing with {} and with non heap variables
fn main() {
    {
        // let p = x;
        let p: &i32;
        let d;
        {
            let mut x = 5; //Defining a variable
            d = x; //Copying that variable (No ownership transfer), now both x and d are the owners.
        }
    }
    //There must be borrows here WHY? we wanna check the rule "If owner dies, borrows aren't usable" rule, in action.
    //How do i borrow? which variable from what?
    // println!("This is the reference to x, which is y =  and this is x ofcourse {x}"); //As owner dies already, there is no way to access x (owner) even if the borrow is alive or in the scope.
    // Now we wanna laern about ownership transfer, or is it just copying of the data.
} //This is a compilable code
