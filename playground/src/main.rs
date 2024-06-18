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

// fn main() {
//     // let work = Point { x: 5.0, y: 4.0f32 };
//     // println!("{:#?}", work)
// }

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }

// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     let mut borrows_mutably = || list.push(7);

//     // println!("{list:?}");
//     borrows_mutably();
//     println!("After calling closure: {list:?}");
// }

// use std::thread;

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     thread::spawn(move || println!("From thread: {list:?}"))
//         .join()
//         .unwrap();
// }

// fn main() {
//     let b = Box::new(5);
//     println!("b = {b}");
// }

// #[derive(Debug)]
// enum List<T> {
//     Cons(T, Box<List<T>>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     println!("{:?}", list);
// }

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// use std::ops::Deref;

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn hello(name: &str) {
//     println!("Hello, {name}!");
// }

// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m);
// }

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };
//     drop(c);
//     println!("CustomSmartPointers created.");
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}