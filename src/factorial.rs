//Computing factorials
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}
fn main() {
    let res == factorial(4.0);
    println!("The factorial is {}", n);
}