use std::cmp::min;
use std::fs::File;

use rand::Rng;

use crate::sorting::{insertion, selection};

mod sorting;

fn main() {
    let mut rng = rand::thread_rng();
    let selection_file: File = File::create("./sorted/selection.json").unwrap();
    let insertion_file: File = File::create("./sorted/insertion.json").unwrap();
    let length = 0..1000;
    let mut rand_vec: Vec<i32> = (length.clone()).map(|_| rng.gen_range(length.clone())).collect();
    let mut selection_vec = rand_vec.clone();
    selection(&mut selection_vec, Option::Some(selection_file));
    let mut insertion_vec = rand_vec.clone();
    insertion(&mut insertion_vec, Option::Some(insertion_file));
}
