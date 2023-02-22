fn main() {
    let tup:(i32,f64,u8) = (500,6.4,1);
    let (x,y,z) = tup; //destructuring
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", tup.1); //accessing tuple elements
}
