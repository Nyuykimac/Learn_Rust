fn main() {
    println!("The first letter of the alphabet is {} and the last is {}", "A", 'Z'); //Testing Macros in rust

    let test_value = 10;

    let test_value = test_value * 2;
    let test_value = test_value + 5;

    println!("The new value of the shadowed variable is, {}", test_value); //illustration of concept of shadowing

    let number = 10; // this type is intrinsic ie it will be inferred by the compiler

    let number: i32 = number + 25; // this is an explicit declaration

    println!("the value of the number is : {}", number);
}
