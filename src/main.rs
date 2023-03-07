#![allow(unused)]

use rand::{Rng, random};
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let random_num: u8 = rand::thread_rng().gen_range(1..300);
    println!("Random number : {}", random_num);
}
