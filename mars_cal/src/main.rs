mod scructs;
use scructs::planet::{GravityOnPlanets, Planet};
use std::io;

fn main() {
    let planet_moon = Planet::new("Moon".to_string(), 1.662, 1);
    let planet_mercury = Planet::new("Mercury".to_string(), 3.7, 2);
    let planet_venus = Planet::new("Venus".to_string(), 8.87, 3);
    let planet_mars = Planet::new("Mars".to_string(), 3.77, 4);
    let planet_jupyter = Planet::new("Jupyter".to_string(), 24.79, 5);
    let planet_neptune = Planet::new("Neptune".to_string(), 11.15, 6);
    let planet_uranus = Planet::new("Uranus".to_string(), 8.69, 7);
    let planet_saturn = Planet::new("Saturn".to_string(), 10.44, 8);
    println!("Select the planet?");
    println!("1. Mercury");
    println!("2. Moon");
    println!("3. Venus");
    println!("4. Mars");
    println!("5. Jupyter");
    println!("6. Neptune");
    println!("7. Uranus");
    println!("8. Saturn");
    println!("10 EXIT the program");

    loop  {
        let mut selected_planet = String::new();
        println!("Enter the number of the planet:");
        io::stdin().read_line(&mut selected_planet).unwrap();
        let planet_id: i32 = selected_planet.trim().parse().unwrap();

        if planet_id == 10 {
            println!("You are exiting the program");
            break;
        }
        let mut weight = String::new();
        println!("Enter your current weight:");
        io::stdin().read_line(&mut weight).unwrap();

        let weight_number: f32 = weight.trim().parse().unwrap();

        match planet_id {
            1 => {
                let w = planet_moon.cal_gravity_on_planet(weight_number);
                println!(
                    "Yot weight in {} is: {}",
                    planet_moon.get_name(),
                    w
                )
            },
            2 => {
                let w = planet_mercury.cal_gravity_on_planet(weight_number);
                println!(
                    "Yot weight in {} is: {}",
                    planet_mercury.get_name(),
                    w
                )
            },
            3 => {
                let w = planet_venus.cal_gravity_on_planet(weight_number);
                println!(
                    "Yot weight in {} is: {}",
                    planet_venus.get_name(),
                    w
                )
            },
            4 => {
                let w = planet_mars.cal_gravity_on_planet(weight_number);
                println!(
                    "Yot weight in {} is: {}",
                    planet_mars.get_name(),
                    w
                )
            },
            5 => {
                let w = planet_jupyter.cal_gravity_on_planet(weight_number);
                println!(
                    "Yot weight in {} is: {}",
                    planet_jupyter.get_name(),
                    w
                )
            },
            6 => {
                let w = planet_neptune.cal_gravity_on_planet(weight_number);
                println!(
                    "Yot weight in {} is: {}",
                    planet_neptune.get_name(),
                    w
                )
            },
            7 => {
                let w = planet_uranus.cal_gravity_on_planet(weight_number);
                println!(
                    "Yot weight in {} is: {}",
                    planet_uranus.get_name(),
                    w
                )
            },
            8 => {
                let w = planet_saturn.cal_gravity_on_planet(weight_number);
                println!(
                    "Yot weight in {} is: {}",
                    planet_saturn.get_name(),
                    w
                )
            },
            _ => println!("The planet number you selected is not exist"),
        }
    }



}
