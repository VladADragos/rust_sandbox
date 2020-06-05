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
