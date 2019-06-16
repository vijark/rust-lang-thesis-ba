mod shape {
    use std::fmt;

    #[derive(PartialEq)]
    pub struct Rectangle {
        pub name: String,
        length: f64,
        width: f64,
    }

    impl Rectangle {
        pub fn new(length: f64, width: f64) -> Self {
            Rectangle {
                name: String::new(),
                length: length,
                width: width,
            }
        }

        pub fn length(&self) -> &f64 {
            &self.length
        }

        pub fn width(&self) -> &f64 {
            &self.width
        }

        pub fn length_mut(&mut self) -> &mut f64 {
            &mut self.length
        }

        pub fn width_mut(&mut self) -> &mut f64 {
            &mut self.width
        }

        pub fn perimeter(&self) -> f64 {
            2.0 * self.length + 2.0 * self.width
        }

        pub fn area(&self) -> f64 {
            self.length * self.width
        }
    }

    impl fmt::Display for Rectangle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}{{Length: {}, Width: {}}}",
                self.name, self.length(), self.width()
            )
        }
    }
}

fn main() {
    // create objects with constructor
    let mut first = shape::Rectangle::new(5.0, 2.5);
    let mut second = shape::Rectangle::new(8.0, 18.9);

    // access and modify data marked as public
    first.name = String::from("Alice");
    second.name = String::from("Bob");

    // print objects
    println!(
        "First rectangle: {}\nArea: {}, Perimeter: {}\n",
        first,
        first.area(),
        first.perimeter()
    );

    println!(
        "Second rectangle: {}\nArea: {}, Perimeter: {}\n",
        second,
        second.area(),
        second.perimeter()
    );

    // compare objects
    if first == second {
        println!("Rectangles are equal");
    } else {
        println!("Rectangles are not equal")
    }

    println!("\nchanging values...");

    first.name = second.name.clone();

    // get mutable reference instead of setter method
    *first.length_mut() = 8.0;
    *second.width_mut() = 2.5;

    if first == second {
        println!("Rectangles are equal");
    } else {
        println!("Rectangles are not equal")
    }
}
