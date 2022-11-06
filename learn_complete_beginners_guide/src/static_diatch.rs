///Static Dispatch
///

trait Duplicateble {
    fn dubl(&self) -> String;
}

impl Duplicateble for String {
    fn dubl(&self) -> String {
        format!("{0}-{0}", *self)
    }
}

impl Duplicateble for i32 {
    fn dubl(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplicate<T: Duplicateble>(x: T) {
    println!("{}", x.dubl())
}

pub fn static_dispatch() {
    //!# Static Dispatch
    //! A Generic trait will be converted to the required type at compile time.
    //! - Monomorphization
    //!     Converting to one form

    let a = 42;
    let b = "Hi John".to_string();
    duplicate(a);
    duplicate(b);
}
