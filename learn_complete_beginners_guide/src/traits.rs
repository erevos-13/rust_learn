///Traits
/// is like a interface
///
use std::ops::Add;
struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello World!")
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }
    fn say_hello(&self) {
        println!("println!(\"Hello world\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Java"
    }
    fn say_hello(&self) {
        println!("System.out.printlin...");
    }
}
pub fn traits_fn() {
    //! #Traits
    //!
    //! The list:
    //! - Traits
    //! - Generics
    //! - dyn
    //! - Operation overloading
    //! - Static dispatch
    //! - Dynamic dispatch
    //!
    //! ## Traits
    //! Similar to an interface or abstract class
    //!
    //! Add a definition to a structure
    //! ```
    //! trait Name {
    //! fn must_implement(&self) -> i32;
    //! fn do_action(&self) {...}
    //! fn do_non_instance_action() {...}
    //! }
    //! ```
    //!
    //! Can have definition only or default implementation
    //!
    //! Can have instance and non-instance action
    //!
    //! **Implement a trait**
    //! ```
    //! impl Name for Person {
    //! fn must_implement(&self)-> {42}}
    //! fn new (name: &str) -> Person{
    //!     Person{name: name}
    //! }
    //! ```
    //!
    //! **Can provide a constructor**
    //! ```
    //! trait Name {
    //!     fn new(name: &str) -> Self
    //! }
    //! ```
    //!
    //! - Use it
    //! `let john = Person::new("John")`
    //!
    //!

    let r = RustDev::new(true);
    let j = JavaDev::new(false);
    println!("{}", r.language());
    println!("{}", j.language());
    r.say_hello();
    j.say_hello();

    generic_traits();

    returning_traits();
    added_traits();
    operator_overload();
}

trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str,
}

struct Cat {
    color: &'static str,
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking", self.species);
    }
}

fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark());
}
fn generic_traits() {
    //!# Generics can be limited by traits
    //! ```
    //! fn color<T: Colorable> (a: T) {
    //! ...
    //! }
    //! ```
    //!

    let dog = Dog {
        species: "retriver",
    };
    let cat = Cat { color: "black" };
    bark_it(dog);
}
//Returning trait.

trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog {
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}
impl Animal for Cat {
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}
fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new(Dog { species: "yolo" })
    } else {
        Box::new(Cat { color: "red" })
    }
}
fn returning_traits() {
    //!# We can not return trait because we have memory leaks.
    //!
    //! The Compile needs to know the space required for a function return type
    //!
    //! A Workarounf is to return a box with a dyn trait
    //!
    //! ```
    //! fn get_animal() -> Box<dyn Animal> {
    //!     ...
    //! }
    //! ```
    //!
    //! dyn is a new addition to the language old code might not have it
    //!
    println!("The animal says :{}", get_animal(0.5).make_noise());
    println!("The animal says :{}", get_animal(2.5).make_noise())
}
// Added traits

trait Summble<T> {
    fn sum(&self) -> T;
}
impl Summble<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}
///Added Traits
fn added_traits() {
    //! # Traits Added
    //! We can add a trait to structure we didn't create.
    //! ```
    //! impl My_trait for Vec<i32> {
    //!     ...
    //! }
    //! ```

    let a = vec![1, 2, 3, 4, 5];
    println!("sum = {}", a.sum()); // the sum are available now
    let b = vec![1.0, 2.0, 3.0];
    // println!("sum = {}", b.sum()) // this is not work because only work in i32.
}
///Operation Overload

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn operator_overload() {
    //!# We can implement standard operation for custom
    //! `use std::ops::Add;`
    //!
    //! ```
    //! struct Custom {
    //!  ...
    //! }
    //!
    //! impl Add for Custom {
    //!  type Output = Custom;
    //! fn add(self: Custom, other: Custom) -> Custom {
    //!     ...
    //! }
    //!
    //! }
    //! ```

    let a = Point { x: 3.4, y: 4.6 };
    let b = Point { x: 1.3, y: 4.5 };
    let c = a + b;
    println!("Added operator overload: {:?}", c);
}
