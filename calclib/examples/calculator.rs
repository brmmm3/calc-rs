use std::env;
use std::io::Error;

use calclib::Calc;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut calc = Calc::new(0.0);
    for arg in args.iter() {
        if arg.starts_with("+") {
            match arg.parse::<f64>() {
                Ok(v) => {
                    calc.add(v);
                }
                Err(e) => eprintln!("Failed to convert {arg}: {e}"),
            }
        } else if arg.starts_with("-") {
            match arg.parse::<f64>() {
                Ok(v) => {
                    calc.sub(-v);
                }
                Err(e) => eprintln!("Failed to convert {arg}: {e}"),
            }
        }
    }
    println!("Result={}", calc.result());
    Ok(())
}
