use std::io;

fn main() {
    let n = loop {
        println!("Enter a positive integer:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read from stdin!");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input; try again.");
                continue;
            }
        };
        if input <= 0 {
            println!("Number must be positive; try again.");
            continue;
        }
        break input;
    };

    println!("Calculating the n-th Fibonacci number, n = {}", n);
    let result = nth_fib(n);
    println!("Your Fibonacci number is {}", result)
}

fn nth_fib(n: u8) -> u128 {
    if n < 3 {
        return 1;
    }
    let mut num_old: u128 = 1;
    let mut num_new: u128 = 1;
    for _ in 3..=n {
        let num_next = num_old.checked_add(num_new).expect("Integer overflow! Number is too big.");
        num_old = num_new;
        num_new = num_next;
    }
    num_new
}
