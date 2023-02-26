struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
//Unit Like Structs
//structs that don't have any fields are called unit-like structs
//they can be useful in situations in which you need to implement a trait on some type but don't have any data that you want to store in the type itself
//the () type is called unit type, and we can use unit-like structs without having to give them names

//struct AlwaysEqual;
//fn main(){
//    let a = AlwaysEqual;
//}





