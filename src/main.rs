fn coins( input: u32 ) -> Vec<u32> {
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
    output
}



fn main() {
    let output = coins( 26u32 );

    println!("{:?}", &output);
}