use std::{
    fs::File,
    io::{self, Read},
};

///? Operator
pub fn question_operator() {
    //!# ? Operation
    //! A shorthand for an entire match statement
    //! ```
    //! let mut f = File::open("example.txt")?;
    //! ```
    //! We want the result if only exist.

    let e = read_username_from_file();
    println!("{:?}", e);
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("src/error_handling/username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("src/error_handling/username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
