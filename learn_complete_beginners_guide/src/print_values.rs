//! Printing Values
pub fn print_valuesFn() {
    //! # Simple values passing values

    println!("Hello, world!");
    // valsue passing
    println!("my name is {} and i am {} years old", "Orfeas", 37);
    println!("a + b= {}", 6 + 3);
    println!("{0} has a {2} value {1} ", "Orfeas", "car", "rust");
    /// key value pairs
    println!(
        "{name} and {surname}",
        surname = "Voutsaridis",
        name = "Orfeas"
    );
    /// Printing traits
    println!("binary {:b}, Hex: {:x}, Octal: {:o}", 50, 50, 50);
    /// Debug
    println!("array is going to be: {:?}", [1, 2, 3, 4])
}
