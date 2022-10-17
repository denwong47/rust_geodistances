mod input_output;
use input_output::pickle::traits::{PickleImport, PickleExport};
mod data;
mod geodistances;
use geodistances::traits::{CalculateDistance, CheckDistance};

use rand::Rng;
#[derive(Copy, Clone, Debug)]
struct DistanceCompare{
    s: data::structs::LatLng,
    e: data::structs::LatLng,
    haversine: f64,
    cartesian: f64,
    vincenty: f64,
}

#[allow(unused_variables)]
fn main() -> Result<(), ()> {
    let stdin_input = input_output::stdin::get_stdin();

    let array_inputs = data::structs::IOCoordinateLists::from_pickle(&stdin_input);

    let array_outputs = data::structs::IOResultArray::like_input(&array_inputs);

    // input_output::stdout::display_bytes(&array_inputs.to_pickle());


    fn calculate(x:u32) -> DistanceCompare {
        let mut rng = rand::thread_rng();

        let point1 = data::structs::LatLng::new(
            rng.gen_range(-15.0..15.0),
            rng.gen_range(-30.0..30.0),
        );
        let point2 = data::structs::LatLng::new(
            rng.gen_range(-15.0..15.0),
            rng.gen_range(-30.0..30.0),
        );

        return DistanceCompare{
            s: point1,
            e: point2,
            haversine: geodistances::Haversine::distance(&point1, &point2).unwrap(),
            cartesian: geodistances::Cartesian::distance(&point1, &point2).unwrap(),
            vincenty: geodistances::Vincenty::distance(&point1, &point2).unwrap(),
        }
    }
    let _results:Vec<DistanceCompare> = (0..100).map(calculate).collect();

    for _result in _results {
        println!("{:?}", _result);
    }

    return Ok(());
}
