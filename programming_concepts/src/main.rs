use std::io;

fn main() {
    shadowing();
    numbers();
    booleans();
    characters();
    tuple_1();
    tuple_2();
    // let (index, element) = access_array([1,2,3,4,5]);
    // println!("The value of the element at index {index} is: {element}");

    conditional(22);
    loop_with_return_var();
    loop_labels();
    while_loop();
    loop_over_array();
    loop_over_numbers();
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn numbers() {
    let mut unsigned_int: u32 = 22;
    println!("{unsigned_int}");
    
    unsigned_int = 432;
    println!("{unsigned_int}");

    // this will cause an error
    // unsigned_int = -1;
    
    // this will cause an error
    // let n_float: f64 = 2;
    
    let n_float: f64 = 2.2;
    println!("{n_float}");
    
    // this will cause an error
    // n_float = -1.3

    let n_float = -33.1;
    println!("{n_float}");

    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{quotient}");
    println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");
}

fn booleans() {
    let t = true;
    println!("{t}");

    let f: bool = false; // with explicit type annotation
    println!("{f}");
}

fn characters() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} {z} {heart_eyed_cat}");
}

fn tuple_1() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let a = tup.0;
    println!("{a}");
    
    // will cause an error
    // println!("{tup}");
    // println!("{tup.0}");
}

fn tuple_2() {
    let tup = (500, 6.4, 1);

    let (x, y, _z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn access_array(a: [i32; 5]) -> (usize, i32){
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    if index > (a.len() - 1) {
        println!("Index too high");
        return access_array(a);
    };

    let element = a[index];
    return (index, element);
}

fn conditional(number: u16) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loop_with_return_var() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!("The counter is {counter}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // break inner loop
                break;
            }
            if count == 2 {
                // break labeled outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_over_array() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_over_numbers() {
    for number in (0..5) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}