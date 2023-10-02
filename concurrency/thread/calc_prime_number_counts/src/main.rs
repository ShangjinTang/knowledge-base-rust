use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let args = std::env::args();
    let nthreads = if args.len() > 1 {
        i32::from_str_radix(&args.last().unwrap(), 10).unwrap()
    } else {
        4
    };
    println!("Using {} threads", nthreads);

    let prime_counts = calc_prime_counts(nthreads, 1, 100);
    println!("Prime counts: {}", prime_counts);
}

fn calc_prime_counts(n: i32, first: i32, last: i32) -> usize {
    let data = Vec::new();
    let lock = Arc::new(Mutex::new(data));

    let mut handles = Vec::new();
    let chunk = (last - first + 1) / n;

    for i in 0..n {
        let start = chunk * i + first;
        let end = if i == n - 1 { last } else { start + chunk - 1 };
        let clone = lock.clone();
        let handle = thread::spawn(move || {
            for i in start..=end {
                if is_prime(i) {
                    let mut v = clone.lock().unwrap();
                    v.push(i);
                }
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    let data = lock.lock().unwrap();
    data.len()
}

fn is_prime(n: i32) -> bool {
    let limit = f64::from(n).sqrt() as i32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}
