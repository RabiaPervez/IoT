1- Download rust from rust-lang.org </br>
2- to check the version use rustc --version or write cargo in terminal to see if it is available</br>

3- open vscode</br>
4- then install extensions (rust analyzer and rust extension pack)</br>
5- make program main.rs and write this prog in it</br>
fn main() {</br>
println!("Hello world");</br>
}</br></br>

6- now go in terminal and type rustc main.rs</br>
7- 2 files will be generated, i.e., main.exe and main.pdb</br>
8- now use .\main.exe to execute the file</br></br>

Rust’s naming convention for constants is to use all uppercase with underscores between words.</br>

### SHADOWING:</br>
Shadowing is the declaration of a new variable with the same name as the previous variable, and the new variable shadows the previous variable.</br>
A variable can be shadowed with a new type. E.g. String can be shadowed as an integer.</br>

### DIFFERENCE BETWEEN SHADOWING AN MUTABILITY</br>
The difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.</br>

