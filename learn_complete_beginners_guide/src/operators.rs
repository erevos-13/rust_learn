/// Operations
pub fn operationFn() {
    //! # Operators
    //! `+,*,/,-,%`
    //!
    //! ## Increment(++) & Decrement(--)
    //! - **Are not Supported**
    //!
    //! ## Relational
    //! `>, >=, <, <=, ==, !=`
    //!
    //! Same as typescript.
    //!
    //! ## Logical
    //! - `&&` AND
    //! - `||` OR
    //! - `!` NOT
    //!
    //! ## Bitwise
    //! - `&` Bitwise AND
    //! ```
    //! 10 & 3 = 2
    //! ```
    //! - `|`  Bitwise OR
    //! ```
    //! 10 | 3 = 11
    //! ```
    //! - `^`  Bitwise XOR
    //! ```
    //! 10 ^ 3 = 9
    //! ```
    //! - `!` Bitwise NOT
    //! ```
    //! !10 = -11
    //! ```
    //! - `<<` Left shift
    //! ```
    //! 10<< 1 = 20
    //! ```
    //! - `>>` Right shift
    //! ```
    //! 10 >> 1 = 5
    //! ```

    let a = 8 + 8;
    let b = 10 / 2;
    let c = 10 % 3;
    println!("a={}, b={}, b={}", a, b, c);
    println!("{}", a >= b && b > c);
}
