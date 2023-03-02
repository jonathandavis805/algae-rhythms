use std::fs::File;
use std::io::Write;

use serde_json;

pub fn selection(unsorted: &mut Vec<i32>, save: Option<File>) {
    let length = unsorted.len();
    let mut list = Vec::new();
    for x in 0..length {
        selection_step(unsorted, x, length);
        maybe_add(&save, &mut list, unsorted)
    }
    maybe_save(save, list)
}

fn maybe_add(save: &Option<File>, list: &mut Vec<Vec<i32>>, unsorted: &Vec<i32>) {
    match save {
        None => {}
        Some(_) => {
            list.push(unsorted.clone())
        }
    }
}

fn maybe_save(save: Option<File>, list: Vec<Vec<i32>>) {
    match save {
        None => {}
        Some(mut file) => {
            file.write(&serde_json::to_string(&list).unwrap().as_bytes()).expect("TODO: panic message");
        }
    }
}

fn selection_step(unsorted: &mut Vec<i32>, x: usize, length: usize) {
    let mut min_index = x;
    for y in x..length {
        if unsorted[y] < unsorted[min_index] {
            min_index = y
        }
    }
    unsorted.swap(x, min_index);
}

pub fn insertion(unsorted: &mut Vec<i32>, save: Option<File>) {
    let length = unsorted.len();
    let mut list = Vec::new();
    for x in 1..length {
        insertion_step(unsorted, x);
        maybe_add(&save, &mut list, unsorted)
    }
    maybe_save(save, list);
}

fn insertion_step(unsorted: &mut Vec<i32>, x: usize) {
    let mut i = x;
    while i > 0 && unsorted[i] < unsorted[i - 1] {
        unsorted.swap(i, i - 1);
        i = i - 1;
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::sorting::{insertion, selection};

    #[test]
    fn test_insertion_dont_move() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 1, 2, 3, 5, 6];
        insertion(&mut test_vec, None);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_insertion() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 1, 3, 5, 6];
        insertion(&mut test_vec, None);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_insertion_dup() {
        let immut_vec = vec![0, 2, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 2, 3, 5, 6];
        insertion(&mut test_vec, None);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_large_insertion_against_builtin() {
        let mut rng = rand::thread_rng();
        let length = 0..10000;
        let mut rand_vec: Vec<i32> = (length.clone()).map(|_| rng.gen_range(length.clone())).collect();
        let mut test_vec = rand_vec.clone();
        let builtin_start = chrono::Local::now();
        rand_vec.sort();
        let builtin_done = chrono::Local::now();
        let impl_start = chrono::Local::now();
        insertion(&mut test_vec, None);
        let impl_done = chrono::Local::now();
        println!("builtin took {}", builtin_done - builtin_start);
        println!("implementation took {}", impl_done - impl_start);
        assert_eq!(rand_vec, test_vec)
    }

    #[test]
    fn test_selection_dont_move() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 1, 2, 3, 5, 6];
        selection(&mut test_vec, None);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_selection() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 1, 3, 5, 6];
        selection(&mut test_vec, None);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_selection_dup() {
        let immut_vec = vec![0, 2, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 2, 3, 5, 6];
        selection(&mut test_vec, None);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_large_against_builtin() {
        let mut rng = rand::thread_rng();
        let length = 0..10000;
        let mut rand_vec: Vec<i32> = (length.clone()).map(|_| rng.gen_range(length.clone())).collect();
        let mut test_vec = rand_vec.clone();
        let builtin_start = chrono::Local::now();
        rand_vec.sort();
        let builtin_done = chrono::Local::now();
        let impl_start = chrono::Local::now();
        selection(&mut test_vec, None);
        let impl_done = chrono::Local::now();
        println!("builtin took {}", builtin_done - builtin_start);
        println!("implementation took {}", impl_done - impl_start);
        assert_eq!(rand_vec, test_vec)
    }
}
