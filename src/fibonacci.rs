/// Calculate fibonacci number by recursive calls.
///
/// # Examples
/// ```
/// use fib_bench_test::fibonacci;
///
/// let result = fibonacci::fibonacci_rec(10);
/// assert_eq!(result, 55);
/// ```
///
pub fn fibonacci_rec(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
    }
}

/// Calculate fibonacci number by loop.
///
/// # Examples
/// ```
/// use fib_bench_test::fibonacci;
///
/// let result = fibonacci::fibonacci_loop(10);
/// assert_eq!(result, 55);
/// ```
///
pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let temp = a + b;
            a = b;
            b = temp;
        }

        a
    }
}

mod fibonacci_tests {
    #[cfg(test)]
    mod fibonacci_rec_tests {
        use crate::fibonacci;

        #[test]
        fn test_fibonacci_zero() {
            let result = fibonacci::fibonacci_rec(0);
            assert_eq!(result, 0)
        }

        #[test]
        fn test_fibonacci_ten() {
            let result = fibonacci::fibonacci_rec(10);
            assert_eq!(result, 55)
        }
    }

    #[cfg(test)]
    mod fibonacci_loop_tests {
        use crate::fibonacci;

        #[test]
        fn test_fibonacci_zero() {
            let result = fibonacci::fibonacci_loop(0);
            assert_eq!(result, 0)
        }

        #[test]
        fn test_fibonacci_ten() {
            let result = fibonacci::fibonacci_loop(10);
            assert_eq!(result, 55)
        }
    }
}
