#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 22);
    }

    #[test]
    #[should_panic]
    fn panic_test1() {
        panic!();
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn panic_test2() {
        let v = vec![1, 2, 3];
        let n = v[10];
    }
}
