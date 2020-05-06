use std::io::{self, Read};
use std::result::Result;

fn gcd(a: u32, b: u32) -> u32 {
    match (a, b) {
        (_, 0) => a,
        (_, _) => gcd(b, a % b),
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    // let numbers = Result::<Vec<i32>, _>::from_iter(
    //     buffer
    //     .split_whitespace()
    //     .map(|s| s.parse::<i32>()).collect::<Vec<Result<_, _>>>());
    let numbers = buffer
        .split_whitespace()
        .map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>().unwrap();
    let s = numbers
        .iter().sum::<i32>();
    println!("inputs: {:?}", numbers);
    println!("sum: {}", s);
    println!("gcd: {}", gcd(1200, 246));
    return Ok(())
}
