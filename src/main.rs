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

fn fibonacci(n: u32) -> u64 {
    let mut n = n;
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut n1 = 0;
        let mut n2 = 1;
        let mut n3 = 1;
        while n >= 2 {
            n = n - 1;
            n3 = n2 + n1;
            n1 = n2;
            n2 = n3;
        }
        n3
    }
}
