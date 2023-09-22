// // fn main() {
// //     let first_name: &str = "Alok";
// //     let last_name: &str = "Karn";
// //     display(first_name, last_name);
// // }

// // fn display(first_name: &str, last_name: &str) {
// //     println!("Hello, My name is {} {}", first_name, last_name);
// // }

// // Integers Numbers::
// // i8, i16, i32, i64, i128, isize --> depends on computer architecture
// // u8, u16, u32, u64, u128, usize --> depends on computer architecture

// // Floating Point Numbers:
// // f32, f64

// // INTEGER

// // fn main() {
// //     let x: i32 = 5;
// //     let mut y = 5;

// //     y = x;

// //     let z = 10; // i32 by default

// //     println!("Success!");
// // }

// // fn main() {
// //     let v: u16 = 38_u8 as u16;
// //     println!("v: {}", v);
// // }

// // Tips:
// // if we don't explicitly assign a type to variable, then the compiler will infer one for us.

// // fn main() {
// //     let x: i32 = 5;
// //     assert_eq!("i32".to_string(), type_of(&x));

// //     println!("Success!");
// // }

// // fn type_of<T>(_: &T) -> String {
// //     format!("{}", std::any::type_name::<T>())
// // }

// // fn main() {
// //     assert_eq!(i8::MAX, 127);
// //     assert_eq!(u8::MAX, 255);

// //     println!("Success!");
// // }
// /*
//     ############# to be continued #############
// */
// // fn main() {
// //     let v1 = 251_u8 + 8;
// //     let v2 = i8::checked_add(251, 8).unwrap();
// //     println!("v1: {}, v2: {}", v1, v2);

// // }

// /*
//     ############# to be continued #############
// */
// fn variable() {
//     // chat gpt
//     // variables
//     let age: i32 = 30;

//     let pi: f64 = 3.14159265359;

//     let is_rust_fun: bool = true;

//     let heart_emoji: char = '❤';

//     let greeting: &str = "Hello, Rust!";

//     print!(
//         "age: {}, \npi: {}, \nis_rust_fun: {}, \nheart_emoji: {}, \ngreeting: {}",
//         age, pi, is_rust_fun, heart_emoji, greeting
//     );

//     println!("\n\nSuccess!");
// }

// fn ownership_borrowing_lifetime() {
//     // ownership, borrowing and lifetime

//     /*
//        Ownership:
//            - Rust's ownership system is a unique feature that ensures memory safety
//                and prevents data races.
//            - Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.
//            - Rust's ownership system has a set of rules that the compiler checks at compile time.
//            - Each value in Rust has a variable that's its "owner". When the owner goes
//            out of the scope, the value will be deallocated.
//        Borrowing:
//            - Instead of transferring ownership, borrowing allows Rust to borrow a reference to a value to use it within a scope.
//            - Borrowing is a way to use a value without taking ownership of it.
//        Lifetime:
//            - Rust uses lifetimes to track how long references are valid. This ensures that references don't outlive the data they point to.

//     */
//     // example::

//     let s1 = String::from("Hello"); // s1 owns the String
//     let s2 = s1; // Ownership of the string moves to s2 (s1 is no longer valid)
//     let s3 = &s2; // s3 borrows the string from s2 (immutable borrow)

//     let mut s4 = String::from("World");
//     let s5 = &mut s4; // s5 borrows the string from s4 (mutable borrow)
//                       // print!("s1: {}", s1);
//     println!("s2: {}", s2);
//     println!("s3: {}", s3);
//     // println!("s4: {}", s4);
//     println!("s5: {}", s5);
// }

// // CONTROL FLOW

// fn control_flow() {
//     /*
//        Rust provides standard control flow constructs like:
//            - if, else, while and for
//        examples::
//     */
//     let number: i32 = 42;

//     if number < 0 {
//         println!("{} is negative", number);
//     } else if number > 0 {
//         println!("{} is positive", number);
//     } else {
//         println!("{} is zero", number);
//     }

//     let mut counter: i32 = 0;

//     while counter < 5 {
//         println!("counter: {}", counter);
//         counter += 1;
//     }

//     for i in 0..5 {
//         println!("value: {}", i);
//     }

//     let array: [i32; 5] = [1, 2, 3, 4, 5];

//     for element in array.iter() {
//         println!("element: {}", element);
//     }

//     println!("Success!");
// }

// // STRUCTURES:: Structures are used to create custom data types

// struct Person {
//     name: String,
//     age: u32,
// }
// fn person_struct() {
//     let person: Person = Person {
//         name: String::from("Alice"),
//         age: 30,
//     };

//     println!("Name: {}", person.name);
//     println!("Age: {}", person.age);
// }

// // ENUMS:: Enumerations allows you to define a type that can have one of several different values.

// enum Weather {
//     Sunny,
//     Cloudy,
//     Rainy,
//     Snowy,
// }
// fn enum_function() {
//     let today: Weather = Weather::Sunny;
//     let tomorrow: Weather = Weather::Rainy;

//     // Match on enum variants to perform different actions

//     match today {
//         Weather::Sunny => println!("It's sunny today!"),
//         Weather::Cloudy => println!("It's cloudy today!"),
//         Weather::Rainy => println!("It's rainy today!"),
//         Weather::Snowy => println!("It's snowy today!"),
//     }

//     match tomorrow {
//         Weather::Sunny => println!("It's sunny tomorrow!"),
//         Weather::Cloudy => println!("It's cloudy tomorrow!"),
//         Weather::Rainy => println!("It's rainy tomorrow!"),
//         Weather::Snowy => println!("It's snowy tomorrow!"),
//     }
// }
use std::io;
fn main() {
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);
    // SHADOWING
    // let x: i32 = 5;

    // let x: i32 = x + 1;

    // {
    //     let x: i32 = x * 2;
    //     // println!("The value of x in the inner scope is: {}", x);
    // }

    // println!("The value of x in the outer scope is: {}", x);

    // let spaces: &str = "   ";
    // let spaces: usize = spaces.len();

    // println!("spaces: {}", spaces);

    // let mut new_spaces = "   ";
    // new_spaces = new_spaces.len();

    // println!("new_spaces: {}", new_spaces);

    // DATA TYPES:
    // Scalar Types: integers, floating-point numbers, Booleans, and characters
    // Compound Types: tuples and arrays

    // Scalar Types:
    // let guess: u32 = "42".parse().expect("Not a number");
    // println!("guess: {}", guess);

    // Floating point numbers

    // let x: f32 = 2.0; // f32
    // let y: f64 = 3.0; // f64

    // println!("x: {}, y: {}", x, y);

    // arithmetic();

    // BOOLEAN TYPE

    // let t: bool = true;
    // let f: bool = false;

    // println!("t: {}, f: {}", t, f);

    // if !t {
    //     println!("t is true");
    // } else {
    //     println!("t is false");
    // }

    // CHARACTER DATA TYPE

    // let c: char = 'a';
    // let heart_emoji: char = '❤';
    // println!("c: {}, heart_emoji: {}", c, heart_emoji);

    // COMPOUND TYPES
    /*
        Compound types can group multiple values into one type. Rust has two primitive compound types:
            - tuples
            - arrays
    */
    // TUPLE TYPE
    /*
        - A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
        - Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    */

    // example::

    // let tup: (i32, f64, u8) = (101, 5.6, 1);
    // let (x, y, z) = tup;
    // println!("x: {}, y: {}, z: {}", x, y, z);

    // println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);

    // ARRAY TYPE

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let b: [i32; 5] = [1; 5];

    // println!("a: {:?}", a);
    // println!("b: {:?}", b);

    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // let first = arr[0];
    // println!("first: {}", first);

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Enter a number between 1 and 12");

    // for month in months.iter() {
    //     println!("{}", month);
    // }
    let mut index: String = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let value: &str = months[index];
    println!("The value of the index is: {}", value);
}

// fn arithmetic() {
//     // addition
//     let sum: i32 = 5 + 10;
//     println!("sum: {}", sum);

//     // subtraction
//     let difference: f32 = 95.5 - 4.3;
//     println!("difference: {}", difference);

//     // multiplication
//     let product: i32 = 4 * 30;
//     println!("product: {}", product);

//     // division
//     let quotient: f32 = 56.7 / 32.2;
//     println!("quotient: {}", quotient);

//     // remainder
//     let remainder: i32 = 43 % 5;
//     println!("remainder: {}", remainder);
// }
