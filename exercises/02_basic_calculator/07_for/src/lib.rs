// Rewrite the factorial function using a `for` loop.
// nacho's note: My code may be uglier, but the loop iterates 1 less time than the solution heheheh
pub fn factorial(mut n: u32) -> u32 {
    if n == 0 {
        return 1
    }
    for i in 1..n{
        n *= i;
    }
    n
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
