use std::io;

fn main() {
        
    loop {
        println!("Please input a positive integer!");

        let mut input_str = String::new();

        io::stdin().read_line(&mut input_str)
            .expect("Failed to read line!");
    
        let input_num: u32 = match input_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let res = fibonacci(input_num);

        println!("The nth Fibonacci number is {}", res);
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
