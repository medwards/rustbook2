use std::env;

fn main() {
    let target: u32 = match env::args().nth(1).unwrap().trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Fibonacci number index starts at 1");
            return;
        }
    };
    let mut fib_last: u64 = 0;
    let mut fib: u64 = 1;
    match target {
        0 => {
            println!("Fibonacci number index starts at 1");
            return;
        }
        1 => {
            println!("Fibonacci number {} is {}", target, fib_last);
            return;
        },
        2 => {
            println!("Fibonacci number {} is {}", target, fib);
            return;
        },
        _ => ()
    };

    let mut fib_count = 2;
    while fib_count < target {
        let fib_old = fib;
        fib = match fib.checked_add(fib_last) {
            Some(num) => num,
            None => {
                println!("Numbers got too big: Fibonacci number {} is {}", fib_count, fib);
                return;
            }
        };
        fib_last = fib_old;
        fib_count = fib_count + 1;
    }
    println!("Fibonacci number {} is {}", target, fib);
}
