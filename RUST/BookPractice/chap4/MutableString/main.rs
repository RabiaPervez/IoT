fn main() {
   let mut s=String::from("Hello");
   s.push_str(",world!"); //push_str appends a literal to a string
   println!("{}",s); //to print 'Hello, world!'
}
//Rust calls a special function called drop to return the memory after the variable goes out of scope
