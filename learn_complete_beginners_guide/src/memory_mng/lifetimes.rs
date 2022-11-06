///Lifetime
///
#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person, //lifetime parameter
}

impl Person {
    fn get_name(&self) -> &String {
        &self.name
    }
}

pub fn lifetime_fn() {
    //!# Lifetime
    //! An indication of how long an object will live.
    //! ```
    //! fn get_string() -> &'static str {
    //! "Hello"
    //!}
    //! ```
    //!
    //! This function is return static str and we
    //! know when the fn is ends this str will destroy.
    //!
    //! ## Rust prevents parts of objects outliving the object.
    //! ```
    //! struct Object<'lifetime_name> {
    //!     field: &'lifetime_name str
    //! }
    //! ```
    //!
    //! ## Lifetime elision - compiler builds lifetimes for us when evident

    println!("{}", get_string());
    let p1 = Person {
        name: String::from("Orfeas"),
    };
    let d1 = Dog {
        name: String::from("Nero"),
        owner: &p1,
    };
    println!("lifetime, {:?}", d1);

    let mut a: &String;
    {
        let p2 = Person {
            name: String::from("orfeas"),
        };
        // a = p2.get_name(); //borrowed value does not live long enough
        a = p1.get_name()
    }
    println!("{}", a);
}

fn get_string() -> &'static str {
    "Hello"
}
