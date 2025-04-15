pub mod fibonacci;

fn main() {
    println!("Fibonacci_rec(10) = {}", fibonacci::fibonacci_rec(10));
    println!("Fibonacci_loop(10) = {}", fibonacci::fibonacci_loop(10));
}
