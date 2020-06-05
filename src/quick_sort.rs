// enum
#[path = "utils.rs"]
mod utils;

use rand;
use std::fmt::{Debug, Display};
use std::time::Instant;
use std::{thread, time};
// mod bubble_sort;
pub fn quick_sort<T: PartialOrd + Copy + Display>(array: &mut [T]) -> &mut [T] {
    sort(array, 0, array.len() - 1);

    return array;
}

fn sort<T: PartialOrd + Copy + Display>(array: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    } else {
        let pivot = partition(array, low, high);
        sort(array, low, pivot - 1);
        sort(array, pivot + 1, high);
    }
}
fn partition<T: PartialOrd + Copy + Display>(array: &mut [T], low: usize, high: usize) -> usize {
    let pivot = *(&array[low]);
    let mut _low = low + 1;
    let mut _high = high;

    while _low <= _high {
        while array[_low] < pivot {
            if _low + 1 == array.len() {
                break;
            }
            _low += 1;
        }

        while array[_high] > pivot {
            if (_high - 1) == low {
                break;
            }
            _high -= 1;
        }

        if _low <= _high {
            array.swap(_low, _high);
            if _low > low {
                _low += 1;
            }
            if _high > high {
                _high -= 1;
            }
        }
    }

    array.swap(low, _high);
    return _high;
}

#[test]
fn partition_test() {
    let mut a = [2, 1, 3];
    partition(&mut a, 0, 2);
    assert_eq!(a, [1, 2, 3]);
}

fn gen_array() -> [u32; 5000] {
    let mut test_arr: [u32; 5000] = [0; 5000];

    for i in 0..4999 {
        test_arr[i] = rand::random::<u32>();
    }
    return test_arr;
}

#[test]
pub fn sort_test() {
    let mut arr = gen_array();
    let now = Instant::now();
    quick_sort(&mut arr);
    println!("time elapsed {}", now.elapsed().as_micros());
    drop(now);
    assert!(true);
}
