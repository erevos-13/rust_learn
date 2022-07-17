use std::io;
fn main() {
    println!("ENter your weight (kg):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mut mars_weigth = calculate_weight_on_mars(weight);
    println!("Weight on Mrs: {}kg", mars_weigth);
}
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
fn some_nf(s: &mut String) {
    s.push_str("a");
}`