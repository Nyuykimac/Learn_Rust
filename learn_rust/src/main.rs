fn main() {
    println!("The first letter of the alphabet is {} and the last is {}", "A", 'Z'); //Testing Macros in rust

    let test_value = 10;

    let test_value = test_value * 2;
    let test_value = test_value + 5;

    println!("The new value of the shadowed variable is, {}", test_value); //illustration of concept of shadowing

    let number = 10; // this type is intrinsic ie it will be inferred by the compiler

    let number: i32 = number + 25; // this is an explicit declaration

    println!("the value of the number is : {}", number);

    //Data types in Rust. There are four main data types namely: Integer, Floating, caracter and boolean.

    let int_number = 16u32;
    let int_signed_number = 23u32;
    let sum: u32 = int_number + int_signed_number;

    println!("the sum is: {}", sum);

    // bool type

    let check = 1 > 34;

    println!(" is 1>34?: {}", check);

    //string, there several types of strings but we focus on two types: str and String

    let name = "Mark";

    let last_name: &str = "Ngoran";

    println!("the name is : {}, last name is: {}", name, last_name);
}
