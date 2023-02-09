use std::io;

fn main() {
    let mut num_subjects = 0;
    loop {
        println!("Enter the number of subjects:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        num_subjects = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };
        if num_subjects > 0 {
            break;
        } else {
            println!("Number of subjects must be greater than zero.");
        }
    }

    let mut total_marks = 0;
    for i in 0..num_subjects {
        let mut mark = 0;
        loop {
            println!("Enter marks for subject {}:", i + 1);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            mark = match input.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    continue;
                }
            };
            if mark >= 0 && mark <= 100 {
                break;
            } else {
                println!("Marks must be between 0 and 100 inclusive.");
            }
        }
        total_marks += mark;
    }

    let percentage = (total_marks as f32 / (num_subjects as f32 * 100.0)) * 100.0;
    let grade = if percentage >= 80.0 {
        "A"
    } else if percentage >= 70.0 {
        "B"
    } else if percentage >= 60.0 {
        "C"
    } else {
        "D"
    };

    println!("Percentage: {:.2}%", percentage);
    println!("Grade: {}", grade);
}
