pub fn is_sorted<T: PartialOrd>(array: &[T]) -> bool {
    for i in 0..(array.len() - 1) {
        if array[i] > array[i + 1] {
            return false;
        }
    }

    return true;
}

#[test]
fn is_sorted_test() {
    if is_sorted(&[1, 2, 3]) == false {}
    let mut a = [1, 2, 3];
    assert_eq!(is_sorted(&a), true);
    a.reverse();
    assert_eq!(is_sorted(&a), false);
}
