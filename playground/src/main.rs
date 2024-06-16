// fn main() {
//     println!("result = {}", fun_test(100, 50));
// }

// fn fun_test(i1: i32, i2: i32) -> i32 {
//     i1 + i2
// }

// fn main() {
//     let condition = true;
//     let number = { if condition { 5 } else { 6 } };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     let mut number = 3;

//     while (number != 0) {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     // let s1 = String::from("hello");
//     // let s2 = s1;

//     // println!("{}, world!", s1);

//     // let s1 = String::from("hello");
//     // let s2 = s1.clone();

//     // println!("s1 = {}, s2 = {}", s1, s2);

//     let s = "Hello, world!";
// }

// fn main() {
//     let my_string = String::from("hello world");

//     // `first_word` works on slices of `String`s, whether partial or whole
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // `first_word` also works on references to `String`s, which are equivalent
//     // to whole slices of `String`s
//     let word = first_word(&my_string);

//     let my_string_literal = "hello world";

//     // `first_word` works on slices of string literals, whether partial or whole
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }


// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(rect1);
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: i32 = v[2];
//     println!("The third element is {third}");

//     let third = v.get(2);
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element."),
//     }
// }

// fn main() {
//     // let s1 = String::from("Hello, ");
//     // let s2 = String::from("world!");
//     // let s1 = s1 + &s2; // note s1 has been moved here and can no longer be used

//     // println!("{s1}");

//     // let s1 = String::from("tic");
//     // let s2 = String::from("tac");
//     // let s3 = String::from("toe");

//     // let s = format!("{}-{}-{}", s1, s2, s3);
//     // println!("{s}");
// }

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("foo");
//     // let score = scores.get(&team_name).copied().unwrap_or(0);
//     let score = scores.get(&team_name);
//     let score_value = match score {
//         Some(i) => *i,
//         _ => 0,
//     };

//     println!("{score_value}")
// }
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

fn main() {
    // let work = Point { x: 5.0, y: 4.0f32 };
    // println!("{:#?}", work)
}