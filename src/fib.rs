#![allow(dead_code)]
mod fib {
    use std::collections::HashMap;
    // use std::thread::sleep;
    use std::time::Instant;

    pub fn fib_rec(n: u64) -> u64 {
        if n == 0 {
            1
        } else if n == 1 {
            1
        } else {
            fib_rec(n - 1) + fib_rec(n - 2)
        }
    }

    pub fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
        match n {
            0 | 1 => 1,
            n => {
                if map.contains_key(&n) {
                    *map.get(&n).unwrap()
                } else {
                    let val = fib_dyn(n - 1, map) + fib_dyn(n - 2, map);
                    map.insert(n, val);
                    return val;
                }
            }
        }
    }

    pub fn fib_test(fib_type: &str, len: u64, verbose: bool) {
        let now = Instant::now();
        match fib_type {
            "1" => {
                let mut map = HashMap::new();
                for i in 0..len {
                    fib_dyn(i, &mut map);
                    if verbose {
                        print!("i:{} value: {}\n", i, fib_dyn(i, &mut map));
                        print!("time: {}\n", now.elapsed().as_millis());
                    }
                }
            }
            "2" => {
                for i in 0..len {
                    fib_rec(i);
                    if verbose {
                        print!("i:{} value: {}\n", i, fib_rec(i));
                        print!("time: {}\n", now.elapsed().as_millis());
                    }
                }
            }
            _ => {
                print!("invalid type(2 for recursive, 1 for dynamic)");
            }
        }

        println!("final time: {}", now.elapsed().as_millis());
    }
}
