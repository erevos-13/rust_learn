///Ownership
pub fn onwnership() {
    //!# Only one variable can own a piece of memory.
    //!## For primitive types, copying data is cheep (we know the size so is easy.)
    //!
    //! ```
    //!  let i = 5;
    //!  let j = i;
    //!  println!("{} , {}", i, j)
    //! ```
    //!
    //!## For complex types, ownership is transferred.
    //!
    //! ```
    //! let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    //! let w = v;
    //! println!("Complex types move {:?}, {:?}", v, w); //v -> value borrowed here after move
    //! ```
    //!
    //! - I can transfer the value from a faction like lambda.
    //!
    //! ```
    //!  let foo = |v: Vec<i32>| -> Vec<i32> {
    //! println!("vector used in foo");
    //! v
    //!};

    //!let v = foo(v);
    //! ```

    let i = 5;
    let j = i;
    println!("{} , {}", i, j);

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // let w = v;
    // println!("Complex types move {:?}, {:?}", v, w); //v -> value borrowed here after move
    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("vector used in foo");
        v
    };

    let v = foo(v);
    println!("{:?}", v)
}
