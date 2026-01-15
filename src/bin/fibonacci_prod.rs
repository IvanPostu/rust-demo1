use my_lib::fibonacci::FibonacciSequence;

fn main() {
    let prod: u64 = FibonacciSequence(10).into_iter().skip(1).product();
    println!("Product of fibonacci sequence elements from 2nd to 10th: {prod}");
}
