///Function.

fn say_hi() {
    println!("Hello orfeas");
}
fn loop_say_hi() {
    for i in 1..6 {
        say_hi();
    }
}
fn say_hello(name: &str) {
    println!("Say hello with params: {}", name);
}
fn say_hello_by_ref(name: &mut &str) {
    *name = "Iliana";
    println!("Say hello with params: {}", name);
}
fn say_hello_greeting(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    greeting
}

pub fn functionFn() {
    //! # Functions
    //! use loop for and functions.
    //!
    //! ## Function pass value
    //! ```
    //! fn say_hello(name: &str) {
    //! println!("Say hello with params: {}", name);
    //! }
    //! ```
    //! Function - Pass by reference
    //! `say_hello_by_ref(&mut name);`
    //! ```
    //! fn say_hello_by_ref(name: &mut &str) {
    //!    println!("Say hello with params: {}", name);
    //! }
    //! ```
    //! - variable we can access what points to with the `*`
    //!
    //! `*name = 'Orfeas'`
    //!
    //! name is variable that we pass in function
    //!
    //! ## Function that returns value
    //! ```
    //! fn say_hello_greeting(name: &mut &str) -> String {
    //!   let greeting = format!("Hello {}", name);
    //!   greeting
    //! }
    //! ```
    let mut name = "orfeas";
    say_hi();
    loop_say_hi();
    say_hello(name);
    println!("The name for the print is {}", name);
    say_hello_by_ref(&mut name);
    let greeting = say_hello_greeting(&mut name);
    println!("Greeting : {}", greeting);
}
