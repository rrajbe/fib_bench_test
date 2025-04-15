use fib_bench_test::fibonacci;

#[test]
fn test_fibonacci_rec_zero() {
    let result = fibonacci::fibonacci_rec(0);
    assert_eq!(result, 0)
}

#[test]
fn test_fibonacci_rec_ten() {
    let result = fibonacci::fibonacci_rec(10);
    assert_eq!(result, 55)
}
