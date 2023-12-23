use std::io;

enum Operations {
    Plus, 
    Minus,
    Multiply,
    Divide,
}

fn main() {
    let mut input: String = String::new();

    let mut first_num = String::new();
    let mut second_num = String::new();

    let operation: Operations;

    println!("Please enter an operation (\"Plus\", \"Minus\", \"Multiply\" or \"Divide\"): ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read operation.");

    input = input.trim().to_lowercase();

    io::stdin().read_line(&mut first_num).expect("Failed to read operation.");
    io::stdin().read_line(&mut second_num).expect("Failed to read operation.");

    // Determines which operation the user entered into the String
    if input == "plus" { 
        operation = Operations::Plus; 
    }
    else if input == "minus" {
        operation = Operations::Minus;
    }
    else if input == "multiply" {
        operation = Operations::Multiply;
    }
    else {
        operation = Operations::Divide;
    }

    let first_num: i32 = first_num.trim().parse().unwrap();
    let second_num: i32 = second_num.trim().parse().unwrap();

    match operation {
        Operations::Plus => println!("{}", first_num + second_num),
        Operations::Minus => println!("{}", first_num - second_num),  
        Operations::Multiply => println!("{}", first_num * second_num),  
        Operations::Divide => println!("{}", first_num / second_num),  
    }
}
