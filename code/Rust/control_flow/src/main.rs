fn main() {
    // if-else
    let number = 42;

    if number != 0 {
        println!("number was something other than zero");
    } else {
        println!("number was zero");
    }

    // if-else with return
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // loop with return
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); // the result is 20

    // for-loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() { // (1..4).rev() is the same as (1..=3).rev()
        println!("{}!", number); // 3! 2! 1!
    }
    println!("LIFTOFF!!!");
}
