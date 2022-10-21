use std::cmp;

pub mod haversine;
pub mod vincenty;
pub mod cartesian;
pub mod traits;

pub use haversine::Haversine;
pub use vincenty::Vincenty;
pub use cartesian::Cartesian;

use crate::data::structs::{LatLng, CalculationResult, IOCoordinateLists, IOResultArray};
use traits::{CalculateDistance, CheckDistance, OffsetByVector};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn within_distance_between_two_points<C: CheckDistance>(
    points: (LatLng, LatLng),
    distance: f64,
) -> CalculationResult {
    let (s, e) = points;

    return CalculationResult::WithinDistance(
        C::within_distance(
            &s, &e,
            distance,
        )
    )
}

#[allow(dead_code)]
pub fn offset_by_vector_from_point<C: OffsetByVector>(
    points: (LatLng, LatLng),
    distance: f64,
) -> CalculationResult {
    let (s, e) = points;

    return CalculationResult::WithinDistance(
        C::within_distance(
            &s, &e,
            distance,
        )
    )
}

#[allow(dead_code)]
pub fn distance_map_unthreaded<C: CalculateDistance>(
    input: &IOCoordinateLists,
    origin: (usize, usize),
    size: (usize, usize),
) -> IOResultArray {
    let (x, y) = origin;
    let (a, b) = input.shape();
    let (w, h) = size;

    let upper_x = cmp::min(a, x+w);
    let upper_y = cmp::min(b, y+h);

    // Re-defining size
    let size = (upper_x-x, upper_y-y);

    let [array1, array2] = input.arrays();

    let mut output = IOResultArray::new(size);

    for row in x..upper_x {
        for col in y..upper_y {

            // If there is only one array input, then only calculate the left half
            if input.unique_array_count() > 1 || col<row {
                output.array[row-x][col-y] = distance_between_two_points::<C>(
                    (array1.value()[row], array2.value()[col])
                )
            }
        }
    }

    // If there is only one array input, clone the bottom left half over to the top right
    if input.unique_array_count() == 1 {
        output.mirror_fill(CalculationResult::Geodistance(Some(0.)));
    }

    return output
}
