mod thread;
fn main() {
    let closure1 = |x| {x +1};
    println!("{}", closure1(1));
    thread::handle_thread();
}
