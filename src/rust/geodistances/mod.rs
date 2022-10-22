use std::cmp;

pub mod config;

pub mod haversine;
pub mod vincenty;
pub mod cartesian;
pub mod traits;

pub use haversine::Haversine;
pub use vincenty::Vincenty;
pub use cartesian::Cartesian;

use crate::data::structs::{Bounds, LatLng, CalculationResult, IOCoordinateLists, IOResultArray};
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
    start: LatLng,
    distance: f64,
    bearing: f64,
) -> CalculationResult {

    return CalculationResult::Location(
        C::offset(
            &start,
            distance,
            bearing,
        )
    )
}

#[allow(dead_code)]
pub fn get_distance_bounds_from_point<C: OffsetByVector>(
    start: LatLng,
    distance: f64,
) -> Bounds {
    let mut lng_override: bool = false;

    // Establish Eastern Longitude Bound
    let upper_lng_bound = {
        if let CalculationResult::Location(Some(e_bound)) = offset_by_vector_from_point::<C>(
            start, distance, 90.,
        ) {
            e_bound.lng
        } else {
            180.
        }
    };

    // Establish Western Longitude Bound
    let lower_lng_bound = {
        if let CalculationResult::Location(Some(w_bound)) = offset_by_vector_from_point::<C>(
            start, distance, 270.,
        ) {
            w_bound.lng
        } else {
            -180.
        }
    };

    // Establish Northern Latitude Bound
    let upper_lat_bound = {
        let default = 90.;

        // If our distance required is longer than the distance from start to the North Pole,
        // we risk returning a Latitude < 90 while in reality the line had looped OVER the pole.
        // So we have to detect if we are close to the pole - if we are, then assume max = 90º.
        if let CalculationResult::Geodistance(Some(distance_to_pole)) = distance_between_two_points::<C>(
            (start, LatLng::new(90. - config::EPS, 0.))
        ) {
            // Give it a GENEROUS tolerance.
            // Things close to the poles are WACKY - so if we are anywhere NEAR then we should
            // basically NOT filter.
            if distance_to_pole > distance *2. {
                if let CalculationResult::Location(Some(n_bound)) = offset_by_vector_from_point::<C>(
                    start, distance, 0.,
                ) {
                    n_bound.lat
                } else {
                    default
                }
            } else {
                lng_override = true; // Set it to accept any longitudes - its impossible to bound this
                default
            }
        } else {
            default
        }
    };

    // Establish Southern Latitude Bound
    let lower_lat_bound = {
        let default = -90.;

        // If our distance required is longer than the distance from start to the South Pole,
        // we risk returning a Latitude > -90 while in reality the line had looped OVER the pole.
        // So we have to detect if we are close to the pole - if we are, then assume max = 90º.
        if let CalculationResult::Geodistance(Some(distance_to_pole)) = distance_between_two_points::<C>(
            (start, LatLng::new(-90. + config::EPS, 0.))
        ) {
            // Give it a GENEROUS tolerance.
            // Things close to the poles are WACKY - so if we are anywhere NEAR then we should
            // basically NOT filter.
            if distance_to_pole > distance *2. {
                if let CalculationResult::Location(Some(s_bound)) = offset_by_vector_from_point::<C>(
                    start, distance, 180.,
                ) {
                    s_bound.lat
                } else {
                    default
                }
            } else {
                lng_override = true; // Set it to accept any longitudes - its impossible to bound this
                default
            }
        } else {
            default
        }
    };

    if lng_override {
        return Bounds::new(
            upper_lat_bound,
            180.,
            lower_lat_bound,
            -180.
        )
    } else {
        return Bounds::new(
            upper_lat_bound,
            upper_lng_bound,
            lower_lat_bound,
            lower_lng_bound
        )
    }
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


#[allow(dead_code)]
pub fn within_distance_map_unthreaded<C: CheckDistance>(
    input: &IOCoordinateLists,
    origin: (usize, usize),
    size: (usize, usize),
    distance: f64,
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
        let bounds = get_distance_bounds_from_point::<C>(
            array1.value()[row],
            distance * 1.15  // Extra tolerance - when its non-eucledian geometries
        );

        for col in y..upper_y {

            // If there is only one array input, then only calculate the left half
            if input.unique_array_count() > 1 || col<row {
                output.array[row-x][col-y] = {
                    // Check if the coordinates are within bound
                    if bounds.contains(&array2.value()[col]) {
                        within_distance_between_two_points::<C>(
                            (array1.value()[row], array2.value()[col]),
                            distance
                        )
                    } else {
                        // ** DEBUG PRINT
                        if let CalculationResult::Geodistance(Some(actual)) = distance_between_two_points::<C>(
                            (array1.value()[row], array2.value()[col])
                        ) {
                            if actual <= distance {
                                println!("Filter Mistake detected: {:?}<=>{:?} {:?} {:?}<={:?}",
                                array1.value()[row],
                                array2.value()[col],
                                bounds, actual, distance);
                            }
                        }

                        // Skip the calculation altogether if its outside of the
                        // 4-directions bounds.
                        CalculationResult::WithinDistance(false)
                    }
                }

            }
        }
    }

    // If there is only one array input, clone the bottom left half over to the top right
    if input.unique_array_count() == 1 {
        output.mirror_fill(CalculationResult::Geodistance(Some(0.)));
    }

    return output
}
