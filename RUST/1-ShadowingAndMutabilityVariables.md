1- Download rust from rust-lang.org
2- to check the version use rustc --version or write cargo in terminal to see if it is available

3- open vscode
4- then install extensions (rust analyzer and rust extension pack)
5- make program main.rs and write this prog in it
fn main() {
println!("Hello world");
}

6- now go in terminal and type rustc main.rs
7- 2 files will be generated, i.e., main.exe and main.pdb
8- now use .\main.exe to execute the file

Rust’s naming convention for constants is to use all uppercase with underscores between words.

### SHADOWING:
Shadowing is the declaration of a new variable with the same name as the previous variable, and the new variable shadows the previous variable.
A variable can be shadowed with a new type. E.g. String can be shadowed as an integer.

### DIFFERENCE BETWEEN SHADOWING AN MUTABILITY
The difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

