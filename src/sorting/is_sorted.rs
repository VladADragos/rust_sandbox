pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    return true;
}

#[test]
fn is_sorted_test() {
    let a = [1, 2, 3];
    assert!(is_sorted(&a));
}
