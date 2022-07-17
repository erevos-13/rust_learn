struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}
fn main() {
    let rect1 = Rectangle {
        height: 20,
        width: 20,
    };
    let area_ = rect1.area();
    println!("{}", area_);
}
