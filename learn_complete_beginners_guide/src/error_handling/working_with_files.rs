use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Write};

/// Files
pub fn working_with_files() {
    //!# File
    //!## Create a file:
    //! ```
    //! use std::fs::File
    //! let mut file = File::create("name.txt").expect("failed");
    //! ```
    //!
    //!## Write to a file:
    //! ```
    //! use std::io::Write;
    //! file.write_all("Hello".as_bytes()).expect("failed")
    //! ```
    //!
    //! ## Append content to file
    //!
    //! ```
    //! use std::fs::OpenOption;
    //! let mut file = OpenOption::new().append(true).open("name.txt").expect("failed");
    //! file.write_all(" . world\n".as_bytes()).expect("failed");
    //! ```
    //!
    //! ## Read from a file
    //! ```
    //! use std::io::Read;
    //!  let mut file = File::open("name.txt").unwrap();
    //! file.read_to_string(&mut contents).unwrap();
    //! ```
    //! ## Delete file
    //! ```
    //! use std::fs;
    //! fs::remove_file("name.txt").expect("failed");
    //! ```

    // let mut file = File::create("src/error_handling/name.txt").expect("failed");
    // file.write_all("Hello orfeas".as_bytes()).expect("failed");

    // let mut file = OpenOptions::new()
    //     .append(true)
    //     .open("src/error_handling/name.txt")
    //     .expect("failed");
    // file.write_all(" . world\n".as_bytes()).expect("failed");
    let mut file = File::open("src/error_handling/name.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    remove_file("src/error_handling/name.txt").expect("delete failed");
}
