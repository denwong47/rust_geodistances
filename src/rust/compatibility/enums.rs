use duplicate::duplicate_item;

use ndarray::{
    Ix1,
};

use pyo3::prelude::*;

use ndarray_numeric::{
    ArrayWithBoolIterMethods,

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
    Vincenty,
    config,
};

use super::conversions::{
    BoolArrayToVecIndex,
};

#[pyclass(module="rust_geodistances")]
/// Pseudo-Enum class of all supported calculation models.
pub enum CalculationMethod {
    /// Haversine Calculation Model
    ///
    /// Assumes the Earth as a perfect sphere.
    /// .. note::
    ///     Algorithm derived from
    ///     `Movable Type Scripts <https://www.movable-type.co.uk/scripts/latlong.html>`_
    HAVERSINE,

    // /// Vincenty Calculation Model
    // ///
    // /// Assumes the Earth as an ellpisoid,
    // /// .. note::
    // ///     Algorithm derived from
    // ///     `Movable Type Scripts <https://www.movable-type.co.uk/scripts/latlong-vincenty.html>`_
    VINCENTY,
}
impl Default for CalculationMethod {
    fn default() -> Self { Self::HAVERSINE }
}

pub trait CalculationInterfaceInternal<T> {
    type FnWithinDistance;

    // No Generics on this one.
    fn _distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1;

    fn _distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2;

    fn _distance_within_array(
        &self,
        s:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2;

    fn _displace(
        &self,
        s:&dyn LatLngArray,
        distance:T,
        bearing:T,
        settings: Option<&config::CalculationSettings>,
    ) -> F64LatLngArray;

    fn _within_distance_of_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:T,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray1;

    fn _within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance:f64,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray2;

    fn _within_distance_among_array(
        &self,
        s:&dyn LatLngArray,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray2;

    fn _indices_within_distance_of_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
    ) -> Vec<usize>;

    fn _indices_within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
    ) -> Vec<Vec<usize>>;

}

#[duplicate_item(
    __vector_type__                 __impl_generics__;
    [ f64 ]                         [];
    [ &F64Array1 ]                  [];
    [ &F64ArcArray1 ]               [];
    [ &F64ArrayView<'a, Ix1> ]      [ 'a ];
    [ &F64ArrayViewMut<'a, Ix1> ]   [ 'a ];
)]
impl<__impl_generics__> CalculationInterfaceInternal<__vector_type__> for CalculationMethod {
    type FnWithinDistance = fn(
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance:f64,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray2;

    // No Generics on this one.
    fn _distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1 {
        let f = match self {
            Self::HAVERSINE => Haversine::distance_from_point,
            Self::VINCENTY => Vincenty::distance_from_point,
        };

        return f(s, e, settings);
    }

    fn _distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2 {
        let f = match self {
            Self::HAVERSINE => Haversine::distance,
            Self::VINCENTY => Vincenty::distance,
        };

        return f(s, e, settings);
    }

    fn _distance_within_array(
        &self,
        s:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2 {
        // TODO This is not the intended implentation; this is meant to only calculate
        // the lower half of the grid below the diagonal.
        let f = match self {
            Self::HAVERSINE => Haversine::distance_within_array,
            Self::VINCENTY => Vincenty::distance_within_array,
        };

        return f(s, settings);
    }

    fn _displace(
        &self,
        s:&dyn LatLngArray,
        distance:__vector_type__,
        bearing:__vector_type__,
        settings: Option<&config::CalculationSettings>,
    ) -> F64LatLngArray {
        let f = match self {
            Self::HAVERSINE => Haversine::displace,
            Self::VINCENTY => Vincenty::displace,
        };

        return f(s, distance, bearing, settings);
    }

    fn _within_distance_of_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance: __vector_type__,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray1 {
        let f = match self {
            Self::HAVERSINE => Haversine::within_distance_of_point,
            Self::VINCENTY => Vincenty::within_distance_of_point,
        };

        return f(s, e, distance, settings);
    }

    fn _within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: f64, // Restrict to f64 here
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray2 {
        let f: Self::FnWithinDistance  = match self {
            Self::HAVERSINE => Haversine::within_distance,
            Self::VINCENTY => Vincenty::within_distance,
        };

        return f(s, e, distance, settings);
    }

    fn _within_distance_among_array(
        &self,
        s:&dyn LatLngArray,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray2 {
        let f  = match self {
            Self::HAVERSINE => Haversine::within_distance_among_array,
            Self::VINCENTY => Vincenty::within_distance_among_array,
        };

        return f(s, distance, settings);
    }

    /// Does this belong here, or in lib.rs?
    fn _indices_within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
    ) -> Vec<Vec<usize>> {
        return CalculationInterfaceInternal
               ::<__vector_type__>
               ::_within_distance(
                    self,
                    s, e,
                    distance,
                    settings,
                ).to_vec_of_indices();
    }

    /// Does this belong here, or in lib.rs?
    fn _indices_within_distance_of_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
    ) -> Vec<usize> {
        return self._within_distance_of_point(
            s, e,
            distance,
            settings,
        )
        .indices()
        .to_vec();
    }

}
