use std::cmp;
use std::sync::Arc;
use std::thread;

pub mod config;

pub mod haversine;
pub mod vincenty;
pub mod traits;

pub use haversine::Haversine;
pub use vincenty::Vincenty;

use crate::config::workers_count;

use crate::data::structs::{Bounds, LatLng, CalculationResult, LatLngArraysCompare, CalculationResultGrid};
use crate::data::traits::{Slicable};
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

    // Establish Northern Latitude Bound
    let upper_lat_bound = 'upper_lat_bound: {
        if let CalculationResult::Geodistance(Some(distance_to_pole)) = distance_between_two_points::<C>(
            (start, LatLng::new(90. - config::EPS, 0.))
        ) {
            if distance_to_pole > distance {
                if let CalculationResult::Location(Some(bound)) = offset_by_vector_from_point::<C>(
                    start, distance, 0.,
                ) {
                    // Rust 1.65.0 - named block break
                    break 'upper_lat_bound bound.lat;
                }
            }
        }

        90.
    };

    // Establish Southern Longitude Bound
    let lower_lat_bound = 'lower_lat_bound: {
        if let CalculationResult::Geodistance(Some(distance_to_pole)) = distance_between_two_points::<C>(
            (start, LatLng::new(90. - config::EPS, 0.))
        ) {
            if distance_to_pole > distance {
                if let CalculationResult::Location(Some(bound)) = offset_by_vector_from_point::<C>(
                    start, distance, 180.,
                ) {
                    break 'lower_lat_bound bound.lat
                }
            }
        }
        
        -90.
    };

    // Establish Northern Latitude Bound
    let upper_lng_bound: (f64, f64) = {
        let default:(f64, f64) = (-180., 180.);

        if let (
            CalculationResult::Location(Some(w_bound)),
            CalculationResult::Location(Some(e_bound)),
        ) = (
            offset_by_vector_from_point::<C>(start, distance, 270.),
            offset_by_vector_from_point::<C>(start, distance, 90.),
        ) {
            n_bound.lat
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
            (-180., 180.),
            lower_lat_bound,
            (-180., 180.),
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
pub fn distance_map_sector<C: CalculateDistance, const U:usize, const V:usize, const A:usize, const B:usize>(
    input: &LatLngArraysCompare<A, B>,
    origin: (usize, usize),
) -> CalculationResultGrid<U, V> {
    let (x, y) = origin;

    let upper_x = cmp::min(A, x+U);
    let upper_y = cmp::min(B, y+V);

    // Re-defining size
    let size = (upper_x-x, upper_y-y);

    let [array1, array2] = input.arrays();

    let mut output = CalculationResultGrid::<U,V>::new();

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

    return output
}

/// Unthreaded implementation of distance_map
#[allow(dead_code)]
pub fn distance_map_unthreaded<C: CalculateDistance, const A:usize, const B:usize>(
    input: &LatLngArraysCompare<A, B>,
    max_workers: Option<usize>,
) -> CalculationResultGrid<A, B> {
    assert!(
        max_workers == Some(1) || max_workers == None,
        "When using unthreaded functions, max_workers must be 1; but {:?} found.",
        max_workers
    );

    let mut output = CalculationResultGrid::<A, B>::new();

    output = distance_map_sector::<C, A, B, A, B>(
        &input,
        (0, 0),
    );

    // If there is only one array input, clone the bottom left half over to the top right
    if input.unique_array_count() == 1 {
        output.mirror_fill(CalculationResult::Geodistance(Some(0.)));
    }

    return output
}


/// Threaded implementation of distance_map
#[allow(dead_code)]
pub fn distance_map<C: CalculateDistance, const A:usize, const B:usize>(
    input: &LatLngArraysCompare<A, B>,
    max_workers: Option<usize>,
) -> CalculationResultGrid<A, B> {
    let workers = match max_workers {
        Some(workers) => cmp::min(workers_count(), workers),
        None => workers_count(),
    };

    let mut output = CalculationResultGrid::<A, B>::new();
    let _chunk_w : usize = (A as f32/workers as f32).ceil() as usize;

    // TODO Add Arc into the struct instead
    let input_arc = Arc::new(input.clone());

    let mut handles = Vec::with_capacity(workers);

    for thread_id in 0..workers {
        let input_ref = Arc::clone(&input_arc);

        if (A as i32) - (thread_id as i32) * (_chunk_w as i32) > 0 {
            handles.push(thread::spawn(move || {
                // TODO Actually write this
                distance_map_sector::<C, {_chunk_w}, B, A, B>(
                    &input_ref,
                    (thread_id*_chunk_w, 0),
                )
            }))
        }
    }

    for (thread_id, handle) in (0..handles.len()).zip(handles) {
        if let Ok(result_array) = handle.join() {
            output.sector_replace(
                (thread_id*_chunk_w, 0),
                result_array,
            )
        }
    }

    if input.unique_array_count() == 1 {
        output.mirror_fill(CalculationResult::Geodistance(Some(0.)));
    }

    return output
}


#[allow(dead_code)]
pub fn within_distance_map_sector<C: CheckDistance, const U:usize, const V:usize, const A:usize, const B:usize>(
    input: &LatLngArraysCompare<A, B>,
    origin: (usize, usize),
    distance: f64,
) -> CalculationResultGrid<A, B> {
    let (x, y) = origin;

    let upper_x = cmp::min(A, x+U);
    let upper_y = cmp::min(B, y+V);

    // Re-defining size
    let size = (upper_x-x, upper_y-y);

    let (array1, array2) = input.arrays();

    let mut output = CalculationResultGrid::<A, B>::new();

    for row in x..upper_x {
        let bounds = get_distance_bounds_from_point::<C>(
            array1[row],
            distance * 1.15  // Extra tolerance - when its non-eucledian geometries
        );

        for col in y..upper_y {

            // If there is only one array input, then only calculate the left half
            if input.unique_array_count() > 1 || col<row {
                output.array[row-x][col-y] = {
                    // Check if the coordinates are within bound
                    if bounds.contains(&array2[col]) {
                        within_distance_between_two_points::<C>(
                            (array1[row], array2[col]),
                            distance
                        )
                    } else {
                        // ** DEBUG PRINT
                        // if let CalculationResult::Geodistance(Some(actual)) = distance_between_two_points::<C>(
                        //     (array1.value()[row], array2.value()[col])
                        // ) {
                        //     if actual <= distance {
                        //         println!("Filter Mistake detected: {:?}<=>{:?} {:?} {:?}<={:?}",
                        //         array1.value()[row],
                        //         array2.value()[col],
                        //         bounds, actual, distance);
                        //     }
                        // }

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
