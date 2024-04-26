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

    //compound data type : tuple
    let tuple_content = ("A", 12i32, false);
    println!("tuple is `{}`", tuple_content.0);

    //compound data type:struct

    let student_1 = Student {
        name: String::from("John Doe"),
        address: String::from("Grand North Region"),
        matricule: 'A',
        remote: true,
    };

    struct Student {
        name: String,
        address: String,
        matricule: char,
        remote: bool,
    }

    println!(
        "the struct infos is: {} ,{} ,{} ,{}: ",
        student_1.name,
        student_1.address,
        student_1.matricule,
        student_1.remote
    );
    #[derive(Debug)]
    enum IPkind {
        V6,
        V4,
    }
    #[derive(Debug)]
    #[warn(dead_code)]
    struct IpAddress {
        kind: IPkind,
        address: String,
    }
    let home = IpAddress {
        kind: IPkind::V4,
        address: String::from("123.123.34"),
    };

    let loopback = IpAddress {
        kind: IPkind::V6,
        address: String::from("::1"),
    };
    println!("The ip addresses {:?}  {:?}", home, loopback);

    //declare arrays

    let ar_names = ["John", "Doe"];
    println!("the array is contents in index 0 is: {}", ar_names[0]);
    //declare vector
    let vec_type = vec!["AB", "BC", "CC"];
    print!("the vec is contents in index 0 is: {}", vec_type[0]);
}
