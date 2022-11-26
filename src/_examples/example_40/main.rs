#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Example 40 - Modules
// "crates" - modules that produce a library or an executable.
// "modules" - organize and handle privacy.
// "packages" - build, test, and share crates.
// "paths" - A way of naming and item such as a struct, function

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    order_food();
}

