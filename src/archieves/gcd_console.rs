use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers= Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
                    .expect("err"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The gcd of {:?} is {}", numbers, d);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    let t = a % b;
    return gcd(b, t);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17 ,
            3 * 7 * 11 * 13 *19),
            3 * 11);
    
    assert_eq!(gcd(4, 6), 2)
}

fn main() {
    let gcd = gcd(5, 15);

    println!("{}", gcd);
}
