use std::io;

fn main() {
    let mut input = String::new();
    println!("Add your weight in earth:");
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    dbg!(weight);
    let mars_weight = calculate_on_mars_on_mars(weight);
    println!("mars weight: {}kg", mars_weight);
}

fn calculate_on_mars_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
