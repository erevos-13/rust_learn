use std::io;
const GRAVITY: f32 = 9.81;
#[derive(Debug)]
enum GravityOnPlanets {
    MERCURY,
    MOON,
    VENUS,
    MARS,
    JUPYTER,
    NEPTUNE,
    URANUS,
    SATURN,
}
#[derive(Debug)]
struct Planet {
    id: i16,
    name: GravityOnPlanets,
    gravity: f32,
}

impl Planet {
    fn call_weight_on_planet(&self, weight: &f32) -> f32 {
        (weight / GRAVITY) * self.gravity
    }
}

fn get_planet(Planet { name, gravity, id }: Planet) -> i16 {
    id
}
fn main() {
    let planet_on_moon = Planet {
        id: 1,
        gravity: 1.622,
        name: GravityOnPlanets::MOON,
    };
    let planet_on_mercury = Planet {
        id: 2,
        gravity: 3.7,
        name: GravityOnPlanets::MERCURY,
    };
    let planet_on_venus = Planet {
        id: 3,
        gravity: 8.87,
        name: GravityOnPlanets::VENUS,
    };
    let planet_on_mars = Planet {
        id: 4,
        gravity: 3.711,
        name: GravityOnPlanets::MARS,
    };
    let planet_on_jupyter = Planet {
        id: 5,
        gravity: 24.79,
        name: GravityOnPlanets::JUPYTER,
    };
    let planet_on_neptune = Planet {
        id: 6,
        gravity: 11.15,
        name: GravityOnPlanets::NEPTUNE,
    };
    let planet_on_uranus = Planet {
        id: 7,
        gravity: 8.69,
        name: GravityOnPlanets::URANUS,
    };
    let planet_on_saturn = Planet {
        id: 8,
        gravity: 10.44,
        name: GravityOnPlanets::SATURN,
    };
    let mut planets: Vec<Planet> = vec![
        planet_on_mercury,
        planet_on_moon,
        planet_on_venus,
        planet_on_jupyter,
        planet_on_mars,
        planet_on_neptune,
        planet_on_saturn,
        planet_on_uranus,
    ];
    let mut selected_planet = String::new();

    for iter in &planets {
        println!("Planet id: {:?}, {:?}", iter.id, iter.name);
    }
    println!("Select the planet?");
    io::stdin().read_line(&mut selected_planet).unwrap();
    let planet_id: i16 = selected_planet.trim().parse().unwrap();
    for iter in planets {
        if (iter.id == planet_id) {
            found_plante(iter);
            break;
        }
    }
}
fn found_plante(iter: Planet) {
    let planet_selected = iter;
    let mut input = String::new();
    println!("Selected planet: {:?}", planet_selected);
    println!("Enter your weight on Earth (kg):");
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();

    println!(
        "Weight on {:?} your weight is: {:?}",
        planet_selected.name,
        planet_selected.call_weight_on_planet(&weight)
    );
}
