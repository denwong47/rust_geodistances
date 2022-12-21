// Abstract layer to mimic the public function arguments,
// but uses ndarrays for all parameters and returns.
//
// THIS MODULE IS OBSOLETE - use python.rs instead.

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
use crate::calc_models;

/// Distances mapped between any two pairs of coordinates between `s` and `e`.
pub fn distance(
    s: &F64LatLngArray,
    e: &F64LatLngArray,
    method: Option<&enums::CalculationMethod>,
    settings: Option<&calc_models::config::CalculationSettings>,
) -> F64Array2 {
    let shape = (s.shape()[0], e.shape()[0]);

    return enums::CalculationInterfaceInternal::<&F64Array1>::_distance(
        method.unwrap_or(
            &enums::CalculationMethod::default()
        ),
        s, e,
        shape,
        settings,
    );
}

pub fn distance_from_point(
    s: &F64LatLng,
    e: &F64LatLngArray,
    method: Option<&enums::CalculationMethod>,
    settings: Option<&calc_models::config::CalculationSettings>,
) -> F64Array1 {
    return enums::CalculationInterfaceInternal::<&F64Array1>::_distance_from_point(
        method.unwrap_or(
            &enums::CalculationMethod::default()
        ),
        s, e,
        settings,
    );
}

/// This needs to be refined; there should be a filtering mechanism to
/// remove unnecessary calculations.
pub fn within_distance(
    s: &F64LatLngArray,
    e: &F64LatLngArray,
    distance: f64,
    method: Option<&enums::CalculationMethod>,
    settings: Option<&calc_models::config::CalculationSettings>,
) -> BoolArray2 {
    let shape = (s.shape()[0], e.shape()[0]);

    return enums::CalculationInterfaceInternal::<f64>::_within_distance(
        method.unwrap_or(
            &enums::CalculationMethod::default()
        ),
        s, e,
        distance, shape,
        settings,
    )
}

/// This needs to be refined; there should be a filtering mechanism to
/// remove unnecessary calculations.
pub fn within_distance_of_point(
    s: &F64LatLng,
    e: &F64LatLngArray,
    distance: f64,
    method: Option<&enums::CalculationMethod>,
    settings: Option<&calc_models::config::CalculationSettings>,
) -> BoolArray1 {
    return enums::CalculationInterfaceInternal::<f64>::_within_distance_of_point(
        method.unwrap_or(
            &enums::CalculationMethod::default()
        ), s, e, distance, settings)
}

/// Does this belong here, or in lib.rs?
pub fn indices_within_distance(
    s: &F64LatLngArray,
    e: &F64LatLngArray,
    distance: f64,
    method: Option<&enums::CalculationMethod>,
    settings: Option<&calc_models::config::CalculationSettings>,
) -> Vec<Vec<usize>> {
    return within_distance(
        s, e,
        distance, method,
        settings,
    ).to_vec_of_indices();
}

/// Does this belong here, or in lib.rs?
pub fn indices_within_distance_of_point(
    s: &F64LatLng,
    e: &F64LatLngArray,
    distance: f64,
    method: Option<&enums::CalculationMethod>,
    settings: Option<&calc_models::config::CalculationSettings>,
) -> Vec<usize> {
    return within_distance_of_point(
        s, e,
        distance, method,
        settings,
    )
    .indices()
    .to_vec();
}

pub fn offset() {

}

pub fn offset_from_point() {

}
