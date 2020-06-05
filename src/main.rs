mod bubble_sort;
mod fib;
mod fizzbuzz;
mod guess_game;
mod utils;
mod vec;

fn main() {
    println!("Hello, world!");

    let mut a: [i8; 8] = [1, 4, 3, 5, 7, 1, 0, 2];
    bubble_sort::bubble_sort(&mut a);
    println!("{}", utils::is_sorted(&a));
    // let b = [1, 3, 2];

    // fizzbuzz::fizzbuzz(32);
    // guess_game::guess_game();
}
