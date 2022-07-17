use std::io;
fn main() {
    let name = "orfeas";
    let variable_name: f32 = 5124.7;
    println!("this is my {} and {}", name, variable_name);
    testing();
    sum();
    mutabile();
    flag();
    contisions(5);
    test2();
    loops_fn();
    let sumAll = sum_number(2,2);
    println!("after sum fn with return: {}", sumAll);
}

fn testing() {
    let count = 5;
    let test: i8 = -5;
    let test_positive: u8;
    test_positive = 8;
    println!(
        "This is my variable: {}, and the {} or this {}",
        count, test, test_positive
    );
}

fn sum() {
    let num2: u8 = 10;
    let num1: u8 = 5;
    let sum = num1 + num2;
    println!("the sum of {} and {} is: {}", num1, num2, sum);
}

fn mutabile() {
    let mut x = 5;
    println!("x {}", x);
    x = 6;
    println!("x {}", x);

    const Y: u8 = 10;
    println!("COnst {}", Y);
}

fn flag() {
    let flag_ = 4 <= 3;
    println!("this flag is: {}", flag_);
}
fn contisions(int: u8) {
    if 3 == int {
        println!("is true");
    } else if int >= 2 {
        println!("else if")
    } else {
        println!("is false");
    }
}

fn test2() {
    let flag = false;
    let number = if flag { 10 } else { 1 };
    println!("number {}", number);
}

fn loops_fn() {
    let mut number: i8 = -1;
    while number < 5 {
        number = number + 1;
        println!("the value of number is: {}", number);
    }

    loop {
        println!("looping looping");
        if number == 5 {
            break;
        }
    }
}

fn sum_number(int1: i8, int2: i8) -> i8 {
    let sum = int1 + int2;
    sum
}
