fn selection(unsorted: &mut Vec<i32>) {
    let length = unsorted.len();
    for x in 0..length {
        let mut min_index = x;
        for y in x..length {
            if unsorted[y] < unsorted[min_index] {
                min_index = y
            }
        }
        unsorted.swap(x, min_index);
    }
}

fn insertion(unsorted: &mut Vec<i32>) {
    let length = unsorted.len();
    for x in 1..length {
        let mut i = x;
        while i > 0 && unsorted[i] < unsorted[i - 1] {
            unsorted.swap(i, i - 1);
            i = i - 1;
        }
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
        insertion(&mut test_vec);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_insertion() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 1, 3, 5, 6];
        insertion(&mut test_vec);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_insertion_dup() {
        let immut_vec = vec![0, 2, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 2, 3, 5, 6];
        insertion(&mut test_vec);
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
        insertion(&mut test_vec);
        let impl_done = chrono::Local::now();
        println!("builtin took {}", builtin_done - builtin_start);
        println!("implementation took {}", impl_done - impl_start);
        assert_eq!(rand_vec, test_vec)
    }

    #[test]
    fn test_selection_dont_move() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 1, 2, 3, 5, 6];
        selection(&mut test_vec);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_selection() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 1, 3, 5, 6];
        selection(&mut test_vec);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_selection_dup() {
        let immut_vec = vec![0, 2, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 2, 3, 5, 6];
        selection(&mut test_vec);
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
        selection(&mut test_vec);
        let impl_done = chrono::Local::now();
        println!("builtin took {}", builtin_done - builtin_start);
        println!("implementation took {}", impl_done - impl_start);
        assert_eq!(rand_vec, test_vec)
    }
}
