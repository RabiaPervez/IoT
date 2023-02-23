fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    {
        let r2 = &mut s;
    }
    }
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// if you have a mutable reference to a value, you can have no other references to that value.
//This restrictions prevents data races by refusing to compile code
//Error is also caused while combining mutable and immutable references.
//We also cannot have a mutable reference while we have an immutable one to the same value.
//but these can be used together depending on the scope.

