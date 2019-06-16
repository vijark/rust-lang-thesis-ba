#![allow(dead_code)]

fn main() {
    println!("run \"cargo test\" in this project.");
}

fn increment_mut_ref(n: &mut i32) {
    *n = *n + 1;
}

fn increment_smart_pointer(n: &mut Box<i32>) {
    //   n: &mut Box<i32>
    //  *n: Box<i32>
    // **n: i32
    **n = **n + 1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn primitive_type() {
        // test stack allocated
        let mut x = 5;

        x = x + 1;
        assert_eq!(x, 6);

        super::increment_mut_ref(&mut x);
        assert_eq!(x, 7);

        *&mut x = x + 1;
        assert_eq!(x, 8);

        let mut boxed: Box<i32> = Box::new(x); // x is copied into box because it's a primitive
        super::increment_smart_pointer(&mut boxed);
        assert_eq!(*boxed, 9);

        assert_eq!(x, 8); // Box did not change value x
    }

    #[test]
    fn complex_type() {
        // test heap allocated
        let mut s = String::from("foo");

        // test &mut pointer
        {
            let ptr = &mut s;

            *ptr = String::from("bar");
            assert_eq!(*ptr, "bar");
        }
        assert_eq!(s, String::from("bar"));

        // test smart pointer
        {
            let boxed: Box<&mut String> = Box::new(&mut s);

            **boxed = String::from("baz");
            assert_eq!(**boxed, String::from("baz"));
        }
        assert_eq!(s, String::from("baz"));
    }
}
