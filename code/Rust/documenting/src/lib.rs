//! This is a project to test various features of `rustdoc`.
//! 
//! ## this is a list:
//! - A
//!     - a
//!     - b
//! - B
//! - C
//! 
//! ```should_panic
//! assert!(false);
//! ```

/// # Print hello
/// 
/// this function prints "hello"
pub fn hello() {
    println!("hello");
}

/// # Examples
/// 
/// ```
/// let name = String::from("Max Mustermann");
/// documenting::greet(&name);
/// ```
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

/// Generates the reverse of a string
/// 
/// # Examples
/// ```
/// let s = String::from("lager");
/// let s_rev = documenting::reverse_string(&s);
/// println!("{}", s_rev);
/// # assert_eq!(&s_rev, "regal"); // this line of code is invisible in the documentation
/// ```
pub fn reverse_string(input: &str) -> String {
    String::from(input).chars().rev().collect()
}
