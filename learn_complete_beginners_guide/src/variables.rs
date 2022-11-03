#[allow(unused_variables)]
/// Variables Section
pub fn variableFn() {
    //!# Variables
    //!
    //! This variables can be int, str, bool
    //!
    //! Follow the snake_case naming convention
    //!
    //! Variable is by default immutable
    //! ```
    //! let name = "Orfeas";
    //! let age = 36;
    //! let amount: i64 = 8454143154315454;
    //! let amount_error = 64684615315313513; //Error because is get the base i32 and the number is much more.
    //!
    //! ```
    //! ## Tublet wrap values
    //! ```let (a, b, c) = (2, 5, 4);```
    //!
    //! ## Type Casting
    //! `let pi: f32 = 4.8;`
    //!
    //! ```
    //! let random = 3_55_456; //easy to read value.
    //! ```
    //!
    //! ## Bool
    //! let is_day = true;
    //! let is_night = false;
    //!
    //! ## Emoji
    //! `let smiley_face = '\u{1F601}';`
    //!
    //! ## String slices
    //! `let cat: &str = "Fluffy";`
    //!
    //! ## Static String
    //! `let cat: &'static str = "Fluffy";`
    //! This will live inside of the function or in where they call it.
    //!
    //! String slices are immutable;
    //!
    //! ## String Object
    //! ```
    //! let dog = String::new();
    //! ```
    //! This can be modified.
    //!
    //! ## Format Macro
    //! ```
    //! format!("Hi {} how are you", "Orfeas");
    //! let owner = format!("Hi {} how are you", "Orfeas");
    //! ```
    //! We can pass it like a var.
    //!
    //! ## String Methods
    //! - len
    //! - push & push_str
    let name = "Orfeas";
    let age = 36;
    let amount: i64 = 8454143154315454;
    // Error because is get the base i32 and the number is much more.
    // let amount_error = 64684615315313513; //error.
    let (a, b, c) = (2, 5, 4);
    println!("Variable: {}, {}, {}", a, b, c);

    let pi: f32 = 4.8;
    println!("PI: {}", pi);
    let random = 3_55_456; //easy to read value.
    let is_day = true;
    let is_night = false;
    println!("Day {} and Night{}", is_day, is_night);

    let char1 = 'A';
    println!("A: {}", char1);
    // Emoji
    let smiley_face = '\u{1F601}';
    println!("Emoji: {}", smiley_face);

    // String slices
    let cat1: &str = "Fluffy";
    let cat: &'static str = "Fluffy";

    println!("Cat name:{}", cat);
    let dog = String::new();
    let mut dog = String::from("Max");
    println!("Dog name:{}", dog);

    // Format Macro
    let owner = format!("Hi {} how are you", "Orfeas");
    println!("owner: {}", owner);
    println!(" dog len:{}", dog.len());
    dog.push(' ');
    dog.push_str("the dog");
    println!(" dog len:{} and added string: {}", dog.len(), dog);
    let new_dog = dog.replace("the", "is my");
    println!(
        " dog len:{} and added string: {} and the replace {}",
        dog.len(),
        dog,
        new_dog
    );
}
