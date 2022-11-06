///Data Types
pub fn data_typesFn() {
    //! #Data types
    //! - Arrays
    //! - Vectors
    //! - Slices (Part of array or vector)
    //! - Tuples
    //! - Structures
    //! - Enums
    //! - Generics

    arrays_fn();
    vector_fn();
    slices_fn();
    tuples_fn();
    structures_fn();
    enums_fn();
    generics_fn();
}

fn arrays_fn() {
    //!# Arrays
    //! [i32;3] -> i32 type and 3 the length.
    //!
    //! Static  - cannot be resized
    //!
    //! Element values can be modified but not deleted
    //!
    //! Indexed
    //! ## Array with default values
    //! ```
    //! let mut numbers = [0;15];
    //! const DEFAULT: i32 = 3
    //! let numbers_def = [DEFAULT;15]
    //! ```
    //! update elements - > numbers_def[2] = 4

    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    println!("Primes {:?} and doubles {:?}", primes, doubles);
    let mut numbers = [0; 15];
    const DEFAULT: i32 = 3;
    let mut numbers_def = [DEFAULT; 15];
    println!(
        "Default number: {:?} and number: {:?}",
        numbers_def, numbers
    );
    numbers_def[2] = 4;
    println!("update arrays {:?}", numbers_def);

    for number in numbers_def.iter() {
        println!("the numbers of iterator {}", number);
    }
}

fn vector_fn() {
    //! # Vectors
    //!
    //! Arrays of variable size.
    //!
    //! ## Crate
    //! `Vec::new()`
    //!
    //! or
    //!
    //! `vec![1,2,3]`
    //!
    //! ## Added
    //! in vectors we can added values.
    //! ## Vectors with default values
    //! ```
    //! let mut numbers = vec![2;5];
    //! const DEFAULT :i32 = 6;
    //! let mut numbers = [DEFAULT;8];
    //! ```

    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5];
    println!("Vectors: {:?}", primes);
    primes.push(8);
    println!("Vectors added: {:?}", primes);
    primes.remove(2);
    println!("Vectors removed: {:?}", primes);
    let number_def_1 = vec![2; 5];
    const DEFAULT: i32 = 6;
    let number_def = [DEFAULT; 8];
    println!("Vectors default: {:?}", number_def);
}

fn slices_fn() {
    //! # Slices
    //! A slices is a pointer to a block of memory
    //! ```
    //! let numbers = [1,2,3,4,5];
    //! let slice = &numbers[1..4];
    //! ```
    //!
    //! - Size is determined at runtime
    //! - Can be used on arrays, vectors and strings
    //! - Indexed
    //!
    //! Mutable slices allow us to change values.
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("Slice numbers: {:?}", slice);

    let mut colors = ["red", "green", "blue", "pink"];
    update_colors(&mut colors[2..4]);
    println!("update after fn slices {:?}", colors);
}

fn update_colors(colors_slices: &mut [&str]) {
    //!# Update Slice
    //! We send a ref slice and update this will update the array or vector.
    colors_slices[0] = "yellow";
    colors_slices[1] = "orange";
    println!("Slices update: {:?}", colors_slices);
}

fn tuples_fn() {
    //!# Tuples
    //! A collection of values of various types.
    //! ```
    //! let person = ("John", 27,true);
    //! let person: (&str, i32, bool) = ("John", 27,true);
    //! ```
    //! - Static, cannot be resized
    //! - Element values can be update as they are in the same type
    //! - Indexed
    //! - **Limited to 12 elements**
    //!
    //! ## Accessing elements
    //!
    //! ```
    //!  println!("Tuple:  person:{}, {},{}", person.0, person.1, person.2);
    //!println!(
    //! "Tuple:  person change position:{2}, {1},{0}",
    //! person.0, person.1, person.2
    //!);
    //! ```
    //! ## Update
    //! `person.0 = "Orfeas";`
    //!
    //! ## Destructuring a tuple
    //! `let (name,age,employed) = person`
    //!
    //! the numbers of variables must correspond to number of elements

    let mut person: (&str, i32, bool) = ("John", 27, true);
    println!("Tuple:  person:{}, {},{}", person.0, person.1, person.2);
    println!(
        "Tuple:  person change position:{2}, {1},{0}",
        person.0, person.1, person.2
    );
    person.0 = "Orfeas";
    let (name, age, employed) = person;
    println!(
        "Destructuring element tuple: {}, {}, {}",
        name, age, employed
    );
}

struct Employee {
    name: String,
    company: String,
    age: u32,
}

impl Employee {
    // &self is a pointer to the element of stucture
    fn fn_detail(&self) -> String {
        format!(
            "Mr. {}, age {} , company: {}",
            self.name, self.age, self.company
        )
    }
    fn static_n_detail() -> String {
        String::from("Details of a person")
    }
}
fn structures_fn() {
    //! Structures
    //!
    //! A collection of key value pairs
    //! Is like a key values pair or as a class.
    //!
    //! ```
    //! struct Employee {
    //!     name: String,
    //!     company: String,
    //!     age: u32
    //! }
    //! ```
    //!
    //! ## Adding methods to struct
    //! ```
    //! impl Employee {
    //!     fn fn_detail(&self) -> String{
    //!    format!(
    //!        "Mr. {}, age {} , company: {}",
    //!        self.name, self.age, self.company
    //!       )
    //! }
    //! }
    //! ```
    //! ### Use it:
    //! ```
    //!  println!("Struct : {}, {}, {}", emp1.age, emp1.company, emp1.name);
    //!  println!("Struct fn: {}", emp1.fn_detail());
    //! ```
    //! ## Static methods
    //! A structure can have static methods
    //!
    //!
    //! ```
    //! impl Employee {
    //!     fn static_fn_detail(&self) -> String{
    //!             format!("Mr. {}, age {} , company: {}",self.name, self.age, self.company)
    //!   }
    //! }
    //! ```
    //!
    //! ### Use it:
    //!
    //! `println!("Struct static fn: {}", Employee::static_n_detail());`
    let emp1 = Employee {
        age: 36,
        company: String::from("Amazon"),
        name: String::from("Orfeas"),
    };
    println!("Struct : {}, {}, {}", emp1.age, emp1.company, emp1.name);
    println!("Struct fn: {}", emp1.fn_detail());
    println!("Struct static fn: {}", Employee::static_n_detail());
}

///Enum
#[derive(Debug)]
enum Colors {
    Red,
    Greed,
    Blue,
}
/// Enum Add values
#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
}
///Enum Usage
fn enums_fn() {
    let my_color = Colors::Blue;
    println!("Colors enums {:?}", my_color);
    let person = Person::Name(String::from("Orfeas"));
    println!("Person enum : {:?}", person);
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
enum Colors_with_gen<T> {
    Red(T),
    Blue(T),
    Green(T),
}
///Generics
fn generics_fn() {
    //!# Generics
    //! Allows us to have a variable data types
    //!```
    //! struct Point<T> {
    //!     x: T,
    //!     y: T
    //! }
    //! ```
    //!
    //! Usage
    //! `let p1: Point<i32> = Point {x:6, y: 8};`
    //! `let p2: Point<f64> = Point {x:3.25, y:8.43}`
    //!
    //! ## In Enums
    //! ```
    //! enum Colors<T> {
    //!     Red(T),
    //!     Blue(T),
    //!     Green(T)
    //! }
    //! ```
    //! usage:
    //!
    //! `let color1 = Colors_with_gen::Blue("blue");`
    //!
    //! `let color2 = Colors_with_gen::Red(255);`
    //! ## Multi Generics
    //!```
    //! struct Point<T, V> {
    //!     x: T,
    //!     y: V
    //! }
    //! ```
    //!
    //! usage:
    //! `let p2: Point<i32, f32> = Point { x: 6, y: 8.44 };`

    let p1: Point<i32> = Point { x: 6, y: 8 };
    let p2: Point<f64> = Point { x: 3.25, y: 8.43 };
    println!("P1: {:?}, P2: {:?}", p1.x, p2.x);
    let color1 = Colors_with_gen::Blue("blue");
    let color2 = Colors_with_gen::Red(255);
    println!("Color with enum {:?}, red: {:?}", color1, color2)
}
