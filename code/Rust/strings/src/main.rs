fn main() {
    if let Some(arg1) = std::env::args().nth(1) {
        println!("numbers in arg1: {}", contains_number(&arg1)); // ref to std::string::String is &str
    } else {
        println!("not enough arguments");
    }
}

fn contains_number(input: &str) -> bool {
    input.contains('0')
        || input.contains('1')
        || input.contains('2')
        || input.contains('3')
        || input.contains('4')
        || input.contains('5')
        || input.contains('6')
        || input.contains('7')
        || input.contains('8')
        || input.contains('9')
}
