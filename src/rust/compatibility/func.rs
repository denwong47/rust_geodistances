// Abstract layer to mimic the public function arguments,
// but uses ndarrays for all parameters and returns.

use ndarray_numeric::{
    ArrayWithBoolIterMethods,

    BoolArray1,
    BoolArray2,

    F64Array1,
    F64Array2,
    F64LatLng,
    F64LatLngArray,
};

use super::enums;
use super::conversions::{
    BoolArrayToVecIndex,
};

/// Distances mapped between any two pairs of coordinates between `s` and `e`.
pub fn distance(
    s: &F64LatLngArray,
    e: &F64LatLngArray,
    method: Option<&enums::CalculationMethod>,
    workers: Option<usize>,
) -> F64Array2 {
    let shape = (s.shape()[0], e.shape()[0]);

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

/// This needs to be refined; there should be a filtering mechanism to
/// remove unnecessary calculations.
pub fn within_distance(
    s: &F64LatLngArray,
    e: &F64LatLngArray,
    distance: f64,
    method: Option<&enums::CalculationMethod>,
    workers: Option<usize>,
) -> BoolArray2 {
    let shape = (s.shape()[0], e.shape()[0]);

    return enums::CalculationInterface::<f64>::within_distance(
        method.unwrap_or(
            &enums::CalculationMethod::default()
        ),
        s, e,
        distance, shape, workers,
    )
}

/// This needs to be refined; there should be a filtering mechanism to
/// remove unnecessary calculations.
pub fn within_distance_of_point(
    s: &F64LatLng,
    e: &F64LatLngArray,
    distance: f64,
    method: Option<&enums::CalculationMethod>,
) -> BoolArray1 {
    return enums::CalculationInterface::<f64>::within_distance_from_point(
        method.unwrap_or(
            &enums::CalculationMethod::default()
        ), s, e, distance)
}

/// Does this belong here, or in lib.rs?
pub fn indices_within_distance(
    s: &F64LatLngArray,
    e: &F64LatLngArray,
    distance: f64,
    method: Option<&enums::CalculationMethod>,
    workers: Option<usize>,
) -> Vec<Vec<usize>> {
    return within_distance(
        s, e,
        distance, method, workers,
    ).to_vec_of_indices();
}

/// Does this belong here, or in lib.rs?
pub fn indices_within_distance_of_point(
    s: &F64LatLng,
    e: &F64LatLngArray,
    distance: f64,
    method: Option<&enums::CalculationMethod>,
) -> Vec<usize> {
    return within_distance_of_point(
        s, e,
        distance, method,
    )
    .indices()
    .to_vec();
}

pub fn offset() {

}

pub fn offset_from_point() {

}
