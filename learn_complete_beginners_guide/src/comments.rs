use std::io;

/// Create comment. What i can do.
pub fn commentsMain() {
    // one line comment
    /* multi line com
     * moment
     */

    //! # main fn Heading
    //! this is for every fn comment
    //! ```
    //! fn main () sniped code
    //! ```
    let mut user_input = String::new();
    println!("Add user what: ");
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            println!("You set {}", user_input);
        }
        Err(e) => println!("Error: {}", e),
    }
}
