#![allow(dead_code)]
pub fn bubble_sort<T: PartialOrd>(slice: &mut [T]) {
    let mut i = 1;

    for _ in 0..slice.len() - i {
        bubble(slice);
        i += 1;
    }
}
fn bubble<T: PartialOrd>(slice: &mut [T]) {
    for i in 0..slice.len() - 1 {
        if slice[i] > slice[i + 1] {
            slice.swap(i, i + 1)
        }
    }
}

fn gen_array() -> [u32; 5000] {
    let mut test_arr: [u32; 5000] = [0; 5000];

    for i in 0..4999 {
        test_arr[i] = rand::random::<u32>();
    }
    return test_arr;
}

#[test]
fn bubble_sort_test() {
    use super::is_sorted::is_sorted;
    use std::time::Instant;
    println!("RUNNING BUBBLE SORT TEST");
    let mut arr = gen_array();
    let now = Instant::now();
    bubble_sort(&mut arr);
    println!("time elapsed {}", now.elapsed().as_micros());
    assert!(is_sorted(&arr));
}
