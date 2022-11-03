use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
/// Crates
pub fn crateFn() {
    //! # Crate is away to group modules.
    //!
    //! Multiple modules are grouped into a crate.
    //!
    //! Two types:
    //! - binary crate
    //! - library crate
    //!
    //! Cargo is used to managed crates.
    //!
    //! ## Use it is like
    //! `use crate::crates::arch::arch_files;`
    //! **crates** is my file name. the crate is the base and then we can use the fn in are file were imported.
    //!
    //! ```
    //! use crate::crates::arch::arch_files;
    //! mod crates;
    //! arch_files("orfeas")
    //! ```
    //! Add the mod of the file name.
    //! We can use an change name: `use crate::crates::arch::arch_files as arc` and use it like arch("orfeas")
    //!
    //! ## Crate Library
    //! If we want to add in the project others crates from other people or library we need to added to the
    //! toml (node_modules) like the crate site
    //! then used to the `use rand::Rng;`
    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("rand number {}", a);
    let mut rng_range_value = rng.gen_range(0..10);
    println!("Random number range: {}", rng_range_value);
    // let rand_string: String = thread_rng()
    //     .sample_iter(Alphanumeric)
    //     .take(39)
    //     .collect()
    // println!("Gen string: {}", rand_string)
}

pub mod arch {
    pub fn arch_files(name: &str) {
        println!("Archive file name {}", name);
    }
}
