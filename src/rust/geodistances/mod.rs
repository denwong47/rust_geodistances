pub mod haversine;
pub mod vincenty;
pub mod cartesian;
pub mod traits;

pub use haversine::Haversine;
pub use vincenty::Vincenty;
pub use cartesian::Cartesian;

use crate::data::structs::{LatLng, CalculationResult, IOCoordinateLists, IOResultArray};
use traits::{CalculateDistance, CheckDistance};

pub fn distance_between_two_points<C: CalculateDistance>(
    points: (LatLng, LatLng),
) -> CalculationResult {
    let (s, e) = points;

    return CalculationResult::Geodistance(
        C::distance(
            &s, &e
        )
    )
}

pub fn within_distance_between_two_points<C: CalculateDistance>(
    points: (LatLng, LatLng),
    distance: f64,
) -> CalculationResult {

    if let CalculationResult::Geodistance(Some(measured)) = distance_between_two_points::<C>(points) {
        return CalculationResult::WithinDistance(measured <= distance);
    } else {
        return CalculationResult::WithinDistance(false);
    }
}

pub fn distance_map_unthreaded<C: CalculateDistance>(
    input: &IOCoordinateLists,
) -> IOResultArray {
    let mut output = IOResultArray::like_input(&input);

    let (w, h) = input.shape();
    let [array1, array2] = input.arrays();

    for x in 0..w {
        output.array[x] =   (0..h)
                            .map(
                                |y| distance_between_two_points::<C>(
                                    (array1.value()[x], array2.value()[y])
                                )
                            )
                            .collect()
    }

    return output
}
