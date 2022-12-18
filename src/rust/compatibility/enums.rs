use duplicate::duplicate_item;

use ndarray::{
    Array1,
    Ix1,
};

use pyo3::prelude::*;

use ndarray_numeric::{
    BoolArray1,
    BoolArray2,

    F64Array1,
    F64Array2,
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
    type FnWithinDistance;

    // No Generics on this one.
    fn distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
    ) -> F64Array1;

    fn distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        shape:(usize, usize),
        workers:Option<usize>,
    ) -> F64Array2;

    fn offset(
        &self,
        s:&dyn LatLngArray,
        distance:T,
        bearing:T,
    ) -> F64LatLngArray;

    fn within_distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:T,
    ) -> BoolArray1;

    fn within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance:f64,
        shape:(usize, usize),
        workers:Option<usize>,
    ) -> BoolArray2;
}

#[duplicate_item(
    __vector_type__                 __impl_generics__;
    [ f64 ]                         [];
    [ &F64Array1 ]                  [];
    [ &F64ArcArray1 ]               [];
    [ &F64ArrayView<'a, Ix1> ]      [ 'a ];
    [ &F64ArrayViewMut<'a, Ix1> ]   [ 'a ];
)]
impl<__impl_generics__> CalculationInterface<__vector_type__> for CalculationMethod {
    type FnWithinDistance = fn(
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance:f64,
        shape:(usize, usize),
        workers:Option<usize>,
    ) -> BoolArray2;

    // No Generics on this one.
    fn distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
    ) -> F64Array1 {
        let f = match self {
            Self::HAVERSINE => Haversine::distance_from_point,
            // Self::VINCENTY => Vincenty::distance_from_point,
        };

        return f(s, e);
    }

    fn distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        shape:(usize, usize),
        workers:Option<usize>,
    ) -> F64Array2 {
        let f = match self {
            Self::HAVERSINE => Haversine::distance,
            // Self::VINCENTY => Vincenty::distance,
        };

        return f(s, e, shape, workers);
    }

    fn offset(
        &self,
        s:&dyn LatLngArray,
        distance:__vector_type__,
        bearing:__vector_type__,
    ) -> F64LatLngArray {
        let f = match self {
            Self::HAVERSINE => Haversine::offset_from_point,
            // Self::VINCENTY => Vincenty::offset_from_point,
        };

        return f(s, distance, bearing);
    }

    fn within_distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance: __vector_type__,
    ) -> BoolArray1 {
        let f = match self {
            Self::HAVERSINE => Haversine::within_distance_from_point,
            // Self::VINCENTY => Vincenty::within_distance_from_point,
        };

        return f(s, e, distance);
    }

    fn within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: f64, // Restrict to f64 here
        shape: (usize, usize),
        workers: Option<usize>,
    ) -> BoolArray2 {
        let f: Self::FnWithinDistance  = match self {
            Self::HAVERSINE => Haversine::within_distance,
            // Self::VINCENTY => Vincenty::within_distance,
        };

        return f(s, e, distance, shape, workers);
    }
}
