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

fn main() {}