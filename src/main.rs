use std::io;




fn main() {
    
    fn fib(num: u128) -> u128 {// 011

        if num==0 {
            return 0;
        }else if num==1 {
            return 1;
        }else {
            return fib(num-1) + fib(num - 2)
        }
    }

    println!("Enter the number for the fibonacci: ");
    let mut value = String::new();
    io::stdin()
    .read_line(&mut value)
    .expect("Couldn't read the value");

    let value: u128 = value.trim().parse().expect("Couldn't collect value");

    println!("The fibonacci of {} is: {}", value, fib(value))
}
