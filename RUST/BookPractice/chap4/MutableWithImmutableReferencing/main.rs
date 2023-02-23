fn main() {
    let mut s = String::from("Hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}
