fn main() {
    let s1=String::from("Hello");
    let len=calculate_length(&s1);
    println!("The length of '{}' is {}.",s1,len);
}
fn calculate_length(s:&String)->usize{
    s.len()
}
//The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because
//it does not own it, the value it points to will not be dropped when the reference stops being used.
//When functions have references as parameters instead of the actual values, we won’t
//need to return the values in order to give back ownership, because we never had ownership.

//References are immutable like variables, so we can’t change the value that the reference is pointing to.(until we use mutable referencing)

