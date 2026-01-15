use my_lib::fibonacci::FibonacciSequence;

fn main() {
    let sum: u64 = FibonacciSequence(10).into_iter().sum();
    println!("Sum of first 10 elements of fibonacci sequence: {sum}");
}
