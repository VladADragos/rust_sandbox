// mod bubble_sort;
// mod utils;
// mod fib;
// mod fizzbuzz;
// mod guess_game;
// mod quick_sort;
// mod utils;
// mod vec;
// use bubble_sort::bubble_sort;
// use quick_sort::quick_sort;
// use utils::is_sorted;
mod mat;
mod fib;
mod fizzbuzz;
mod vec;
mod sorting;
fn main() {
    use sorting::quick_sort::quick_sort;
    use sorting::bubble_sort::bubble_sort;
    let mut _a = [4, 3, 7, 2, 100, 0, 50, 10, 15];
    // (&mut a);
    // println!("{:?}", a);
    // sorting::is_sorted::is_sorted();
    quick_sort(&mut _a);
    println!("{:?}",_a);
    // mat::test_mat();
}
