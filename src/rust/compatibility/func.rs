// Abstract layer to mimic the public function arguments,
// but uses ndarrays for all parameters and returns.

use ndarray_numeric::{
    F64Array1,
    F64Array2,
    F64LatLng,
    F64LatLngArray,
};

use super::enums;

/// Distances mapped between any two pairs of coordinates between `s` and `e`.
pub fn distance(
    s: &F64LatLngArray,
    e: &F64LatLngArray,
    method: Option<&enums::CalculationMethod>,
    workers: Option<usize>,
) -> F64Array2 {
    let shape = (s.len(), e.len());

    return enums::CalculationInterface::<&F64Array1>::distance(
        method.unwrap_or(
            &enums::CalculationMethod::default()
        ),
        s, e,
        shape,
        workers,
    );
}

pub fn distance_from_point(
    s: &F64LatLng,
    e: &F64LatLngArray,
    method: Option<&enums::CalculationMethod>,
) -> F64Array1 {
    return enums::CalculationInterface::<&F64Array1>::distance_from_point(
        method.unwrap_or(
            &enums::CalculationMethod::default()
        ),
        s, e,
    );
}

pub fn within_distance() {

}

pub fn within_distance_of_point() {

}

pub fn points_within_distance() {

}

pub fn points_within_distance_of_point() {

}

pub fn offset() {

}

pub fn offset_from_point() {

}
