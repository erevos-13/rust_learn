pub fn playNmae(name: &str) {
    println!("this is the name {}", name);
}

pub fn playingAUdio(name: &str) {
    println!("palying audrio: {}", name);
    clean::perfom_clean();
    clean::files::clean_file();
}
/// Module create
mod clean {
    //! #Create a fn with pub we can use it every were imported.
    //! Second use of the mod inside mod.
    //! ```
    //! pub mod files {
    //!     pub fn clean_file() {
    //!         println!("Removing unused files");
    //!     }
    //! }
    //! ```
    //! To use it wec need to use the hierarchic of nested
    //! `clean::files::clean_file();`
    pub fn perfom_clean() {
        println!("Clean hdd");
    }

    pub mod files {
        pub fn clean_file() {
            println!("Removing unused files");
        }
    }
}
