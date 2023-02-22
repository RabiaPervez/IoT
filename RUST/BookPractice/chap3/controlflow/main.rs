fn main() {
let number = 3;
if number < 5 {
    println!("condition was true");
}else if number > 5 {
    println!("condition was false");
} 
else {
    println!("try something else");
}
if number != 0{
    println!("number was not zero");
}

//using if in let statement
let condition=true;
let num=if condition{5}else{6};
//let num=if condition{5}else{"6"}; won't work as there are different types of variables

println!("The value of num is: {}", num);

//looping
let mut counter=0;
let result = loop{
    counter +=1;
    if counter == 10{
        break counter * 2;
    }
};
println!("The result is: {}", result);

//label loops

let mut count = 0;
'counting_loop:loop{
    println!("counting_loop: {}", count);
    let mut remaining = 10;

    loop{
        println!("remaining: {}", remaining);
        if count == 2{
            break 'counting_loop;
        }
        if remaining == 8{
            break;
        }
        remaining -=1;
    }
    count +=1;
}
println!("End count: {}", count);

//while loop
let mut guess=3;
while guess != 0{
    println!("guess: {}", guess);
    guess -=1;
}
println!("End guess: {}", guess);

//for loop
let a=[10,20,30,40,50];
for element in a{
    println!("the value is: {}", element);
}
//to reverse the range
for numb in (1..4).rev(){
    println!("{}!", numb);
}
println!("done!");

}
