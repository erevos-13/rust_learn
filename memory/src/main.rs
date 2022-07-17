fn main() {
    let mut num1 = 5;
    let num2 = num1;
    num1 = 4;
    println!("num = {} and num2= {}", num1, num2);
    heap();
}

fn heap() {
    let s1 = String::from("hello");
    let s2 = s1; //s1.clone();
    println!("{}", s2);
    heap_in_function();
}

fn heap_in_function() {
    let s1 = String::from("hello");
    let length = calculate_length(&s1);
    println!("the length of {} is:{}",s1, length);
}
fn calculate_length(s: &String) -> (usize) {
    s.len()
}
