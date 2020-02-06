use std::error::Error as stdError;
use std::fmt::{
    Display     as fmtDiplay,
    Formatter   as fmtFormatter,
    Result      as fmtResult
};
use std::result::Result as stdResult;

type Result<T> = stdResult<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmtDiplay for DoubleError {
    fn fmt(&self, f: &mut fmtFormatter) -> fmtResult {
        write!(f, "invalid first item to double")
    }
}
impl stdError for DoubleError {
    fn source(&self) -> Option<&(dyn stdError + 'static)> {
        None
    }
}

fn coins( input: u32 ) -> Result<Vec<u32>> {
    let coins = vec![100, 25, 10, 5, 1];

    let mut output = vec![0, 0, 0, 0, 0];
    let mut somme = input;

    loop {
        let mut index = 0;
        for coin in &coins {
            if &somme >= coin {
                output[index] += 1;
                somme -= coin;
            }
            index += 1;
        }

        if somme <= 0 {
            break;
        }
    }
    Ok(output)
}

fn print(coins: Result<Vec<u32>>) {
    match coins {
        Ok(n) => println!("{:?}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(coins(26u32));
}