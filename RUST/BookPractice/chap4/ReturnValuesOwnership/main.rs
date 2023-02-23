fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello"); //s2 comes into scope
    let s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back, which also moves its return value into s3

}
fn gives_ownership() -> String { //gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); //some_string comes into scope
    some_string //some_string is returned and moves out to the calling function
}
//takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { //a_string comes into scope
    a_string //a_string is returned and moves out to the calling function
}
