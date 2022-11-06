///Borrowing
pub fn borrowing_fn() {
    //!# Borrowing
    //! - Ony one variable can own a piece of memory
    //!
    //! - Variables can borrow ownership to other pieces of memory.
    //!
    //! ```
    //! let a = 6;
    //! let b = &b; //a reference
    //!
    //! println!("{}", *b); //need to add star to use value of the father in this case the a  nad not the reference
    //!
    //! a += 2 //error
    //!
    //! ```
    //! **When the reference is destroy the ownership is return to the father in this case a. **
    //!
    //! **THe borrow has to match the mutability**
    //!
    //! ## Not use
    //! ```
    //! let mut v = vec![1, 2, 3, 4, 5];
    //!for i in &v {
    //! println!("for loop : {}", i);
    //! v.push(6); mutable borrow occurs here
    //! }
    //! ```

    let mut a = 6;
    let b = &mut a;
    println!("{}", *b);
    println!("{}", a);
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("for loop : {}", i);
        // v.push(6); //mutable borrow occurs here
    }
}
