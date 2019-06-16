fn main() {
    let n = 42;

    println!("{0} * {0} = {1}", n, square(n));
    println!("is {} even? {}", n, if is_even(n) { "yes" } else { "no" });
}

fn square(n: i32) -> i32 {
    n * n
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
