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
    //!
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
    functions_and_cope();
    closures_fn();
    higher_order_function();
    macros_fn();
}

static mut R: i32 = 0;
fn functions_and_cope() {
    //! Function and Scope
    //!
    //! ## Scope
    //! No memory leaks - no need to manually deallocate variables.
    //! ```
    //! {
    //! let a = 3
    //! println!("{}",a) //works is the same scope
    //! }
    //! println!("{}",a) //error
    //!```
    //! Global variables can be declared but they are unsafe.
    //! ```
    //! let a = 3;
    //! fn main() {
    //!     unsafe {println!("{}",a)}
    //! }
    //! ```
    //!
    {
        let a = 3;
        println!("{}", a);
    }
    // println!("{}", a); //error
    //unsafe
    unsafe {
        R = 4;
        println!("{}", R)
    }
}

fn closures_fn() {
    //!# Closures
    //! A function within a function
    //!
    //! An anonymous function, lambda expression
    //!```
    //! |a:i32, b: i32| println!("{}", a +b);
    //! |a : i32, b:i32| -> {a + b};
    //! ```
    //! A function van be assigned to variable
    //! ```
    //! let sum |a:i32,b:i32| -> i32 {a + b};
    //! sub(2,3);
    //! ```
    //!
    //! A closure can be generic
    //! ```
    //! let gen = |x| {println!("receive {}",x)}
    //! gen(e)
    //! ```
    //!
    //! - if we try to run after we execute the gen(3) with a
    //!
    //! gen(true) that will not work because the rust recognize the first time tha the `gen` is call and
    //!
    //! expect to be the same

    let a = |a: i32| a + 1;
    println!("{}", a(6));
    let b = |b: i32| -> i32 {
        let c = b + 1;
        c
    };
    println!("closure {}", b(10));
    let gen = |x| println!("receive {}", x);
    gen(2);
    // gen(true); // error
}

fn higher_order_function() {
    //!# HOFs
    //! Functions that take another function as parameter.
    //! This is lambda
    //!
    //! ```
    //!   let square = |a: i32| a * a;
    //! ```
    let square = |a: i32| a * a;
    apply(square, 12);

    // Calculate the sum of all squares less than 500
    // only for even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("The the sum is : {}", sum);

    //With HOFs
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("The sum useing HOFs is {}", sum2);
}
fn is_even(x: i32) -> bool {
    x % 2 == 0
}
fn apply(f: fn(i32) -> i32, a: i32) {
    println!("resut hofs {}", f(a));
}

fn macros_fn() {
    //!# Macros
    //! Write code that writes code - meta programming
    //!
    //! Match an expression and perform some operation
    //!
    //! Code is expanded and compiled
    //! ```
    //! macro_rules! my_macro {
    //! (match) => (code to run)
    //! }
    //! ```
    //!
    //! `println!("This is an {} macro", "orfeas")`
    //!
    //! We can match multiple expressions
    //! ```
    //! marco_ruls! my_macro {
    //! (match1) => (code to run)
    //! (match2) => (code to run)
    //! }
    //! ```
    //!
    //! Designator
    //! - expr
    //! - ident
    //! - block
    //! - stmt
    //! - pat
    //! - path
    //! - meta
    //! - ty
    //! - tt

    macro_rules! my_macro {
        () => {
            println!("first macro")
        };
    }
    // macro_rules! name {
    //     ($name: expr) => {
    //         println!("Hey {}", $name)
    //     };
    // }
    // Multi params
    macro_rules! name {
        ($($name: expr), *) => (
           $( println!("Hey {}", $name);)*
        );
    }
    macro_rules! xy {
        (x => $e: expr) => {
            println!("X is {}", $e)
        };
        (y => $e:expr) => {
            println!("Y is {}", $e)
        };
    }
    macro_rules! build_nf {
        ($fn_name: ident) => {
            fn $fn_name() {
                println!("This is macro fn {:?}", stringify!($fn_name))
            }
        };
    }
    my_macro!();
    name!("Orfeas");
    name!("Orfeas", "yolo", "Jason");
    xy!(x => 5);
    xy!(y => 4);
    build_nf!(hey);
    hey();
}
