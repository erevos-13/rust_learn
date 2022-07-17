use std::{io, collections::HashMap};

fn main() {
    let mut choice = String::new();
    loop {
        calculator();
        println!("Do you want to continue(Y / N)");
        io::stdin()
            .read_line(&mut choice)
            .expect("You did not enter Y or N");
        if choice.trim() == "N" {
            break;
        }
    }
    
}

fn calculator() {
    let mut number1 = String::new();
    let mut number2 = String::new();
    let mut operator = String::new();
    println!("Please enter number 1: ");
    io::stdin()
        .read_line(&mut number1)
        .expect("Fail to read number 1");
    println!("Please enter number 2: ");
    io::stdin()
        .read_line(&mut number2)
        .expect("Fail to read number 2");

    println!("Please enter number Oprator: ");
    io::stdin()
        .read_line(&mut operator)
        .expect("Fail to read operator");

    let num1: u32 = number1
        .trim()
        .parse()
        .expect("Fail to convert string number 1 to number");
    let num2: u32 = number2
        .trim()
        .parse()
        .expect("Fail to convert string number 2 to number");
    let mut result: u32 = 0;
    let selected_operator = operator.trim();
    if selected_operator == "+" {
        result = num1 + num2;
    }
    if selected_operator == "-" {
        result = num1 - num2;
    }
    if selected_operator == "*" {
        result = num1 * num2;
    }
    if selected_operator == "/" {
        result = num1 / num2;
    }

    println!(
        "You select to do {} an the result is:{}",
        selected_operator, result
    );
}
