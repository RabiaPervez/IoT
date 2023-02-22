fn main() {
    let x = 5;
    let x=x+1;
    {
        let x= x+1;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);
    let spaces = "    "; //this is string
    let spaces = spaces.len(); //let allows to change the datatype as well unlike mut, as we get error if try to change data type in mut
    println!("The value of spaces is: {}", spaces);
}
