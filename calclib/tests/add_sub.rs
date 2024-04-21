use std::io::Error;

use calclib::Calc;

#[test]
fn test_add() -> Result<(), Error> {
    let mut calc = Calc::new(1.0);
    let result = calc.add(2.5);
    assert_eq!(3.5, result);
    Ok(())
}

#[test]
fn test_sub() -> Result<(), Error> {
    let mut calc = Calc::new(1.0);
    let result = calc.sub(2.5);
    assert_eq!(-1.5, result);
    Ok(())
}
