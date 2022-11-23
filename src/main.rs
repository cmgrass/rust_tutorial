#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let st3 = String::from("x r t b h k k a m c");
    println!("st3: {}", st3);

    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
      println!("{}", char);
    }

    let st4: &str = "Random string literal"; // String in data segment
    let mut st5: String = st4.to_string(); // Convert to heap
    println!("{}", st4);
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {}", st6.len());

    st5.clear();
    println!("st5 : {}; length {}", st5, st5.len());

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7; // st6 is ditched from memory, st7 stays since we just reference it
    // println!("st6: {}", st6); <-- Errors at compile time, string was moved!
    println!("st7: {}", st7);
    println!("st8: {}", st8);

    for char in st8.bytes() {
      println!("{}", char);
    }
}






// Example 19
// 
// fn main() {
//     let st3 = String::from("x r t b h k k a m c");
//     println!("st3: {}", st3);
// 
//     let mut v1: Vec<char> = st3.chars().collect();
//     v1.sort();
//     v1.dedup();
//     for char in v1 {
//       println!("{}", char);
//     }
// 
//     let st4: &str = "Random string literal"; // String in data segment
//     let mut st5: String = st4.to_string(); // Convert to heap
//     println!("{}", st4);
//     println!("{}", st5);
// 
//     let byte_arr1 = st5.as_bytes();
//     let st6 = &st5[0..6];
//     println!("String length : {}", st6.len());
// 
//     st5.clear();
//     println!("st5 : {}; length {}", st5, st5.len());
// 
//     let st6 = String::from("Just some");
//     let st7 = String::from(" words");
//     let st8 = st6 + &st7; // st6 is ditched from memory, st7 stays since we just reference it
//     // println!("st6: {}", st6); <-- Errors at compile time, string was moved!
//     println!("st7: {}", st7);
//     println!("st8: {}", st8);
// 
//     for char in st8.bytes() {
//       println!("{}", char);
//     }
// }


// Example 18
// 
// fn main() {
//     // Two types of strings:
//     // String - Vector of bytes that can be changed
//     // &str - Pointer to a string and allow for viewing
// 
//     let mut st1 = String::new();
// 
//     st1.push('A');
//     st1.push_str(" bacon");
// 
//     for word in st1.split_whitespace() {
//         println!("{}", word);
//     }
// 
//     let st2 = st1.replace("A", "Another");
//     println!("{}", st2);
// }


// Example 17
// 
// fn main() {
//     let my_tuple: (u8, String, f64) = (47, "Grass".to_string(), 50_000.00);
//     println!("Age : {}", my_tuple.0);
//     println!("Name : {}", my_tuple.1);
//     println!("FloatVal : {}", my_tuple.2);
//     let(v1, v2, v3) = my_tuple;
//     println!("Age : {}", v1);
//     println!("Name : {}", v2);
//     println!("FloatVal : {}", v3);
// }


// Example 16
// 
// fn main() {
//     let arr_2 = [1,2,3,4,5,6,7,8,9];
//     let mut loop_idx = 0;
//     for val in arr_2.iter() {
//       println!("Val : {}", val);
//     }
// }


// Example 15
// 
// fn main() {
//     let arr_2 = [1,2,3,4,5,6,7,8,9];
//     let mut loop_idx = 0;
//     while loop_idx < arr_2.len() {
//       println!("Array : {}", arr_2[loop_idx]);
//       loop_idx += 1;
//     }
// }


// Example 14
// 
// fn main() {
//     let arr_2 = [1,2,3,4,5,6,7,8,9];
//     let mut loop_idx = 0;
//     loop {
//         if arr_2[loop_idx] % 2 == 0 {
//           println!("Not an odd value");
//           loop_idx += 1;
//           continue;
//         }
// 
//         if arr_2[loop_idx] == 9 {
//           println!("no more loopin");
//           break;
//         }
// 
//         println!("odd value : arr_2[{}] == {}", loop_idx, arr_2[loop_idx]); 
//         loop_idx += 1;
//     }
// }


// Example 13
// 
// fn main() {
//     let arr_1 = [1,2,3,4];
//     println!("1st : {}", arr_1[0]);
//     println!("length : {}", arr_1.len());
// }


// Example 12
// 
// fn main() {
//     let my_age = 2;
//     let voting_age = 18;
//     match my_age.cmp(&voting_age) {
//         Ordering::Less => println!("Can't vote"),
//         Ordering::Greater => println!("Can vote"),
//         Ordering::Equal => println!("You gained the right to vote"),
//     };
// }


// Example 11
// 
// fn main() {
//     let age2 = 18;
//     match age2 {
//         1..=18 => println!("Important Birthday"),
//         21 | 50 => println!("Important Birthday2"),
//         65..=i32::MAX => println!("Important Birthday3"),
//         _ => println!("Not an important birthday"),
//     };
// }


// Example 10
// 
// fn main() {
//     let mut my_age = 47;
//     let can_vote = if my_age >= 18 {
//         true
//     } else {
//         false
//     };
//     println!("Can Vote : {}", can_vote);
// }


// Example 9
// 
// fn main() {
//     let age = 100;
//     if (age >= 1) && (age <= 18) {
//       println!("Important Birthday");
//     } else if (age == 21) || (age == 50) {
//       println!("Important Birthday2");
//     } else if age >= 65 {
//       println!("Important Birthday3");
//     } else {
//       println!("Not an important birthday");
//     }
// }


// Example 8
// 
// fn main() {
//     let random_num = rand::thread_rng().gen_range(1..101);
//     println!("Random : {}", random_num);
// }


// Example 7
// 
// fn main() {
//     let mut num_3: u32 = 5;
//     let num_4: u32 = 4;
//     println!("5 + 4 = {}", num_3 + num_4);
//     println!("5 - 4 = {}", num_3 - num_4);
//     println!("5 * 4 = {}", num_3 * num_4);
//     println!("5 / 4 = {}", num_3 / num_4);
//     println!("5 % 4 = {}", num_3 % num_4);
//     num_3 += 1;
//     println!("mut'd num_3 : {}", num_3);
// }


// Example 6
// 
// fn main() {
//     let num_3: u32 = 5;
//     let num_4: u32 = 4;
//     println!("5 + 4 = {}", num_3 + num_4);
//     println!("5 - 4 = {}", num_3 - num_4);
//     println!("5 * 4 = {}", num_3 * num_4);
//     println!("5 / 4 = {}", num_3 / num_4);
//     println!("5 % 4 = {}", num_3 % num_4);
// }


// Example 5
// 
// fn main() {
//     let num_1: f32 = 1.111111111111111;
//     println!("f32 : {}", num_1 + 0.111111111111111);
// 
//     let num_2: f64 = 1.111111111111111;
//     println!("f64 : {}", num_2 + 0.111111111111111);
// }


// Example 4
// 
// fn main() {
//     let is_true = true; // bool type
//     let my_grade = 'A'; // character uses single quotes, string double
// }


// Example 3
// 
// fn main() {
//     // Unsigned integer : u8, u16, u32, u64, u128, usize
//     // Signed integer : i8, i16, i32, i64, i128, isize
// 
//     println!("Max u32 : {}", u32::MAX);
//     println!("Max u64 : {}", u64::MAX);
//     println!("Max usize : {}", usize::MAX);
//     println!("Max u128 : {}", u128::MAX);
//     println!("Max f32 : {}", f32::MAX);
//     println!("Max f64 : {}", f64::MAX);
// }


// Example 2
//
// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32 = 3.141592;
//     let age = "47";
//     let mut age: u32 = age.trim().parse()
//         .expect("Age wasn't assigned a number");
//     age = age + 1;
//     println!("I'm {} and I want ${}", age, ONE_MIL);
// }


// Example 1
//
// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting = "Nice to meet you";
//     io::stdin().read_line(&mut name)
//         .expect("Didn't Receive Input");
// 
//     println!("Hello {}! {}", name.trim_end(), greeting);
// }


