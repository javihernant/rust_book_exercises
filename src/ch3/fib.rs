pub fn fib(nth: i32) -> i32 {
    if nth == 0 {
        return 0;
    }
    if nth == 1 || nth == 2 {
        return 1;
    }
    let mut a = 1;
    let mut b = 1;
    let mut tmp;
    for _ in 2..nth {
        tmp = a;
        a = b;
        b = tmp + b;
    }
    b
}

#[cfg(test)]
mod tests {
    use crate::ch3::fib;
    #[test]
    fn fib_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(10), 55);
    }
}
