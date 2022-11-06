///Reference Counted Variable (Rc Variables)
use ::std::rc::Rc;
struct Car {
    brand: Rc<String>,
}
impl Car {
    fn new(brand: Rc<String>) -> Car {
        Car { brand }
    }
    fn drive(&self) {
        println!("{} is driving", &self.brand);
    }
}

pub fn reference_counted_variables() {
    //!# Rc Variables
    //! A structure that can hold multiple references to a variable
    //!
    //! Can be shared in different places.
    //!
    //! ```
    //! use std::rc::Rc;
    //! fn do_smth(var: Rc<String>) ...
    //!
    //! let var = Rc::new(String::from("test"));
    //!
    //! var.clone()
    //! ```
    //!## Count the variable pointers
    //! ```
    //! Rc::string_count(&var)
    //! ```
    //!

    let brand = Rc::new(String::from("BMW"));
    println!("pointer: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("pointer: {}", Rc::strong_count(&brand));
    }
    println!("my car is a: {}", brand);
    println!("pointer: {}", Rc::strong_count(&brand));
}
