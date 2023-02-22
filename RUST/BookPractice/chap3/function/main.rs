fn main() {
    another_function(5, 'h');
    let x=five();
    println!("The value of x is: {}", x);
    let y=plus_one(5);
    println!("The value of y is: {}", y);
}
fn another_function(value:i32, unit_label:char) {
    println!("The value and unit label is: {}{}", value,unit_label);
}
//Expressions do not include ending semicolons. 
//If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value

//returning values from functions
fn five() -> i32 {
    5
}
fn plus_one(y:i32) -> i32{
    y+1
}
