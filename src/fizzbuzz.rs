#![allow(dead_code)]
mod fizzbuzz {
    pub fn fizzbuzz(n: u64) {
        for i in 1..n {
            print!("i: {}, ", i);
            match (i % 3, i % 5) {
                (0, 0) => println!("fizzbuzz"),
                (0, _) => println!("fizz"),
                (_, 0) => println!("buzz"),
                (_, _) => println!("{}", i),
            }
        }
    }
}
