use std::fs::File;

///Helper methods
pub fn helper_methods() {
    //!# Helper Methods
    //! ## unwrap
    //! Will return the data id it's available or panic! of it's not
    //! ```
    //! File::open("example.txt").unwrap();
    //! ```
    //!## Expect
    //! similar to unwrap but allows us to set custom error message.
    //! ```
    //! File::open("example.txt").expect("Unable to open file");
    //! ```
    //!
    //!
    // let f = File::open("main.jpg").unwrap(); //for not use match.
    // match f {
    //     Ok(f) => println!("file found, {:?}", f),
    //     Err(e) => panic!("file not found"),
    // }
    let f = File::open("main.jpg").expect("unable to open file");
}
