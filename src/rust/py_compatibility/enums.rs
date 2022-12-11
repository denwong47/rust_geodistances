use duplicate::duplicate_item;

use ndarray::{
    Array1,
    Ix1,
};

use pyo3::prelude::*;

use ndarray_numeric::{
    F64Array1,
    F64ArcArray1,
    F64ArrayView,
    F64ArrayViewMut,
    F64LatLngArray,
};

use crate::calc_models::traits::{
    LatLng,
    LatLngArray,
    CalculateDistance,
    OffsetByVector,
    CheckDistance,
};

use crate::calc_models::{
    Haversine,
    // Vincenty,
};

#[pyclass(module="rust_geodistances")]
pub enum CalculationMethod {
    HAVERSINE,
    // VINCENTY,
}
impl Default for CalculationMethod {
    fn default() -> Self { Self::HAVERSINE }
}

pub trait CalculationInterface<T> {
    // No Generics on this one.
    fn distance(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
    ) -> F64Array1;

    fn offset(
        &self,
        s:&dyn LatLngArray,
        distance:T,
        bearing:T,
    ) -> F64LatLngArray;

    fn within_distance(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:T,
    ) -> Array1<bool>;
}

#[duplicate_item(
    VectorType                      Generics;
    [ f64 ]                         [];
    [ &F64Array1 ]                  [];
    [ &F64ArcArray1 ]               [];
    [ &F64ArrayView<'a, Ix1> ]      [ 'a ];
    [ &F64ArrayViewMut<'a, Ix1> ]   [ 'a ];
)]
impl<Generics> CalculationInterface<VectorType> for CalculationMethod {
    // No Generics on this one.
    fn distance(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
    ) -> F64Array1 {
        let f = match self {
            Self::HAVERSINE => Haversine::distance,
            // Self::VINCENTY => Vincenty::distance,
        };

        return f(s, e);
    }

    fn offset(
        &self,
        s:&dyn LatLngArray,
        distance:VectorType,
        bearing:VectorType,
    ) -> F64LatLngArray {
        let f = match self {
            Self::HAVERSINE => Haversine::offset,
            // Self::VINCENTY => Vincenty::offset,
        };

        return f(s, distance, bearing);
    }

    fn within_distance(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:VectorType,
    ) -> Array1<bool> {
        let f = match self {
            Self::HAVERSINE => Haversine::within_distance,
            // Self::VINCENTY => Vincenty::within_distance,
        };

        return f(s, e, distance);
    }
}