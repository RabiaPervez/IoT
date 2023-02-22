use std::io;

fn main() {
    let a:[i32;5] = [1,2,3,4,5]; //array of 5 elements
    let b = [3;5]; //array of 5 elements with all elements having value 3
    let first = a[0]; //accessing array elements

    println!("please enter an array index");
    let mut index=String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index:usize=index
        .trim()
        .parse()
        .expect("Please type a number");
    let element=a[index];
    println!("The value of the element at index {} is: {}", index, element);
}
