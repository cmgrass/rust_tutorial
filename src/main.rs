#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Example 48 - Smart pointers - Box (ctd.)

fn main() {
    // Create a binary tree structure using a box.
    // EX:
    //       50
    //      /  \
    //     35  40
    //    /  \ / \
    //       ...

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key: key,
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self // return self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));

    println!("node1 left: {}", node1.key);

}





// Example 47 - Smart pointers - Box
// 
// fn main() {
//     // Boxes are normally used when you have a lot of data to store in heap.
//     // Pointers to boxes are typically stored on the stack.
// 
//     let b_int1 = Box::new(10); // Now we have a heap allocation
// 
//     println!("b_int1: {}", b_int1);
// }


// Example 46 - Closures ctd. (using generics and passing functions into functions)
// 
// fn main() {
//     fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
//         func(a, b)
//     }
// 
//     let sum = |a, b| a+b;
//     let prod = |a, b| a*b;
// 
//     println!("5 + 4 = {}", use_func(5,4,sum));
//     println!("5 * 4 = {}", use_func(5,4,prod));
// }


// Example 45 - Closures ctd.
// 
// fn main() {
//     let mut sample1 = 5;
//     let print_var = || println!("sample1 = {}", sample1);
//     print_var();
// 
//     sample1 = 10;
//     let mut change_var = || sample1 += 1;
//     change_var();
//     println!("Sample1 = {}", sample1);
// 
//     sample1 = 10;
//     println!("Sample1 = {}", sample1);
// }


// Example 44 - Closures
// 
// fn main() {
//     // Closures are basically functions without names.
//     // They are generally stored as variables.
//     // They can be used to pass a function into another function.
// 
//     // let var_name = |parameters| -> return_type {BODY}
// 
//     let can_vote = |age: i32| {
//         age >= 18
//     };
// 
//     println!("Can vote: {}", can_vote(17));
// }


// Example 43 - Iterators
// 
// fn main() {
//     let mut arr_it = [1,2,3,4];
// 
//     for val in arr_it.iter() {
//         println!("{}", val);
//     }
// 
//     let mut iter1 = arr_it.iter();
// 
//     println!("1st : {:?}", iter1.next());
// }


// Example 42 - File access and error handling
// 
// fn main() {
//     let path = "lines.txt";
//     let output = File::create(path);
// 
//     let mut output = match output {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("Problem creating file : {:?}", error);
//         }
//     };
// 
//     write!(output, "Just some\nRandom words\n").expect("Failed to write to file");
// 
//     let input = File::open(path).unwrap(); // unwrap ignores the result and gives output from file
// 
//     let buffered = BufReader::new(input);
// 
//     for line in buffered.lines() {
//       println!("{}", line.unwrap()); // Again, `unwrap` gives just content instead of `Result` type
//     }
// 
//     let output2 = File::create("rand.txt");
//     let mut output2 = match output2 {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("rand.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Can't create file: {:?}", error),
//             },
//             _other_error => panic!("Problem opening file : {:?}", error),
//         },
//     };
// 
//     write!(output2, "baconator.\n").expect("Failed to write to file");
// }


// Example 41 - Manually panic
// 
// fn main() {
//     panic!("Manually trigger a terrible error, rust will cleanup resources and execution terminates");
// }

