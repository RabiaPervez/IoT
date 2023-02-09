use std::io;

fn main() {
    let mut first_value = String::new();
    let mut second_value = String::new();
    let mut operator = String::new();

    loop {
        println!("Enter the first value:");
        first_value.clear();
        io::stdin().read_line(&mut first_value).unwrap();
        let first_value = match first_value.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("Enter the second value:");
        second_value.clear();
        io::stdin().read_line(&mut second_value).unwrap();
        let second_value = match second_value.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("Enter the operator (+, -, *, /):");
        operator.clear();
        io::stdin().read_line(&mut operator).unwrap();
        let operator = operator.trim();

        let result = match operator {
            "+" => first_value + second_value,
            "-" => first_value - second_value,
            "*" => first_value * second_value,
            "/" => first_value / second_value,
            _ => {
                println!("Invalid operator. Please enter a valid operator (+, -, *, /).");
                continue;
            }
        };

        println!("Result: {}", result);
        break;
    }
}
