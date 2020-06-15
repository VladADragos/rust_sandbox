// mod bubble_sort;
mod fib;
mod fizzbuzz;
mod mat;
mod sorting;
mod vec;
use std::collections::binary_heap;
mod heaps;
fn main() {
    use heaps::comparator::*;
    use heaps::heap::Heap;

    let mut max_heap = Heap::new(MaxComparator::new());
    max_heap.add(100);
    max_heap.add(101);
    max_heap.add(15);
    max_heap.add(150);

    println!("{}", max_heap);
}
