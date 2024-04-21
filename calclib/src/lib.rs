mod calc;
pub use calc::Calc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut calc = Calc::new(2.0);
        let result = calc.add(3.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn sub() {
        let mut calc = Calc::new(2.0);
        let result = calc.sub(3.0);
        assert_eq!(result, -1.0);
    }
}
