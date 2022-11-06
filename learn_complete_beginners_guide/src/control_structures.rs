///Control Structures
pub fn control_structure() {
    if_statement();
    match_statement();
    pattern_matching();
    for_loop();
    while_loop();
}

/// if statement
fn if_statement() {
    //!# if Statement
    //! we have if or else
    let test = false;
    if (test) {
        println!("the value is bool: {}", test);
    }
    if (!test) {
        println!("the value is bool: {}", test);
    }

    let result = if test == true {
        true
    } else if test == false {
        false
    } else {
        false
    };
    println!("The value of the result {}", result)
}
/// Match
fn match_statement() {
    //!# Match like switch

    print_choice(Suit::Heart);
    print_choice(Suit::Club);
    print_choice(Suit::Diamond);
    country(44);
    country(5);
}

enum Suit {
    Heart,
    Space,
    Club,
    Diamond,
}

fn print_choice(choice: Suit) {
    //!# Match with enums
    //! we need to pass all the enums.
    match choice {
        Suit::Heart => println!("\u{2665}"),
        Suit::Space => println!("\u{2660}"),
        Suit::Club => println!("\u{2663}"),
        Suit::Diamond => println!("\u{2666}"),
        _ => println!("No match found"),
    }
}

fn country(code: i32) {
    //!# Match return values
    //!```
    //! let country = match code {
    //! 44 => "UK",
    //! 34 => "Spain",
    //! 1..=99 => "unknown",
    //! _ => "invalid",
    //!};
    //! ```
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=99 => "unknown",
        _ => "invalid",
    };
    println!("Country is {}", country);
}

fn pattern_matching() {
    //!# Pattern matching
    //! - Multiple values with or: `1 | 2`
    //! - Ranges `1..=5`
    //! - Conditions x `if a > b`
    //! - Tuple matching

    for i in 0..=16 {
        println!("I have {} oranges", get_oranges(i));
    }
    let point = (0, 0);
    match point {
        (0, 0) => println!("Points is origin"),
        (x, 0) => println!("Point on x axis({},0)", x),
        (0, y) => println!("y axis (0,{})", y),
        (x, y) => println!("({},{})", x, y),
    }
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an event amount of",
        _ => "lots of",
    };
}

/// For Loop
fn for_loop() {
    //!# For Loop
    //! Loop through a collection or range, execute code for each element
    //! - Continue will skip a step
    //! - Break will stop the loop
    for i in 1..12 {
        println!("{} * {} = {}", i, i, i * i);
    }

    let pets = ["cats", "dogs", "chihuahua", "hamster", "bear"];
    for pet in pets.iter() {
        if pet == &"chihuahua" {
            println!("{} barks too much", pet);
            continue;
        }
        if pet == &"bear" {
            println!("{} is not a pet", pet);
            break;
        }
        println!("I love my {}", pet);
    }

    for (pos, i) in (1..11).enumerate() {
        let squere = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1}", nb, squere)
    }
}

///While loop
fn while_loop() {
    //!# While
    //! Loop as long as a condition is true.
    //! - Continue skips a step
    //! - Break stops the loop
    //!
    //! ## loop
    //! ```
    //! loop {
    //!  ...
    //! }
    //! ```
    get_squares(313543535);
    get_cubes(6465416);
}
fn get_squares(limit: i32) {
    let mut x = 1;
    while x * x < limit {
        println!("{0} * {0} = {1}", x, x * x);
        x += 1;
    }
}
fn get_cubes(limit: i32) {
    let mut x = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", x, x * x * x);
        x += 1;
        if (x * x * x > limit) {
            break;
        }
    }
}
