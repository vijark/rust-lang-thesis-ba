#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }
}

trait Animal {
    fn noise(&self) -> &str;
}

struct Sheep {}

impl Animal for Sheep {
    fn noise(&self) -> &str {
        "baaaaaah"
    }
}

fn animal_noise<T: Animal>(animal: T) {
    println!("This animal makes {}.", animal.noise());
}

fn main() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(1.0, 2.0);
    let p3 = Point::new('x', 'y');

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);

    let animal = Sheep {};
    animal_noise(animal);
}
