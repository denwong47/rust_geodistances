#[cfg(test)]

mod input_output;
mod data;
mod geodistances;
mod config;

use data::traits::Slicable;

#[test]
fn test_unknown() {
    let shape:(usize, usize) = (10, 10);
    let mut array_outputs = data::structs::CalculationResultGrid::<10,10>::new();

    for row in 0..shape.0 {
        for col in 0..shape.1 {
            array_outputs.array[row][col] = data::structs::CalculationResult::Geodistance(
                Some((row*10 + col) as f64)
            )
        }
    }

    println!("{:?}", array_outputs.sector::<3,20>((2,4)));
}
