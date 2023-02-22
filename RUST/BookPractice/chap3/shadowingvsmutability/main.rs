fn main(){
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
    let x=6; // only x = 6 will give compile error
    println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x=6; // by mentioning mut, we can change value of variable x
    println!("The value of x is: {}", x);
