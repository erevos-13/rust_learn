fn main() {
    enum Burger {
        Small,
        Medium,
        Large,
    }
    let burger_size = Burger::Small;
 
    match burger_size {
        Burger::Small => {
            println!("small burger")
        }
        Burger::Medium => {
            println!("md burger")
        }
        Burger::Large => {
            println!("lg burger")
        }
    }

    



}
