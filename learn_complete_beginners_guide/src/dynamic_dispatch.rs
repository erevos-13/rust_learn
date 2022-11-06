///Dynamic dispatch

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

fn duplicate(x: &dyn Duplicateble) {
    println!("{}", x.dubl())
}

pub fn dynamic_dispatch() {
    //!# Dynamic dispatch
    //! A generic trait will be converted to the required type at run time.

    let a = 43;
    let b = "Hi orfeas".to_string();
    duplicate(&a);
    duplicate(&b);
}
