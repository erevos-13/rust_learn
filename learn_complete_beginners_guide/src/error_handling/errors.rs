use std::fs::File;

///Errors
pub fn errors_fn() {
    //!#Errors
    //! ## 2 types of errors
    //! - Recoverable
    //!     - Result enum
    //! ```
    //! enum Result<T,E> {
    //!     Ok(T),
    //!     Err(E)
    //! }
    //! ```
    //!
    //!    - Option enum
    //!
    //!
    //! ```
    //! enum Option<T,E> {
    //!     Some(T),
    //!     None
    //! }
    //! ```
    //!
    //!
    //! - Unrecoverable
    //!     - panic! macro
    //! ```
    //! panic!(message);
    //! ```
    //! Panic will terminate the program or thread.

    // let v = vec![1, 2, 3, 4, 5];
    // v[10];
    // panic!("Something when wrong");
    let file = File::open("main.jpg");
    match file {
        Ok(f) => println!("file found"),
        Err(e) => println!("error: {}", e),
    }
    println!("Continuing on the exec");
    divide(Some(1));
    divide(Some(20));
    divide(None);
    divide(Some(0));
}

const ANSWER_TO_LIFE: i32 = 43;
fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("Cannot divide by zero"),
        Some(x) => println!("Result is : {}", ANSWER_TO_LIFE / x),
        None => println!("None values the answer is {} ", ANSWER_TO_LIFE),
    }
}
