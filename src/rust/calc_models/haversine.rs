use std::f64::consts::PI;
use std::ops::Index;

use duplicate::duplicate_item;

use ndarray::{
    Array1,
    Ix1,
    // Ix2,
    // NdIndex,
};

use super::config::{
    RADIUS,
};

use super::traits::{
    LatLng,
    LatLngArray,
    CalculateDistance,
    OffsetByVector,
    CheckDistance,
};

use ndarray_numeric::{
    ArrayWithF64Methods,
    ArrayWithF64Atan2Methods,
    ArrayWithF64PartialOrd,
    ArrayWithF64AngularMethods,
    ArrayWithF64LatLngMethods,

    BoolArray1,
    ArrayWithBoolIterMethods,
    ArrayWithBoolMaskMethods,

    F64Array1,
    F64ArcArray1,
    F64ArrayView,
    F64ArrayViewMut,
    // F64LatLng,
    F64LatLngArray,
};


/// Haversine calculation
/// Assumes spherical world - fast but has errors up to ~0.35%
/// Adapted from https://towardsdatascience.com/better-parallelization-with-numba-3a41ca69452e
pub struct Haversine;
impl CalculateDistance for Haversine {
    fn distance(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
    ) -> F64Array1 {

        let (s_lat, s_lng) = (s[0], s[1]);

        let (s_lat_r, s_lng_r) = (s_lat * PI / 180., s_lng * PI /180.);

        let e_latlng_r = e.to_rad();
        let (e_lat_r, e_lng_r) = (e_latlng_r.column(0), e_latlng_r.column(1));

        let d = {
            // bear in mind that e_lat_r is an array while s_lat_r is f64
            ((&e_lat_r - s_lat_r)/2.).sin().powi(2)
            + s_lat_r.cos()*e_lat_r.cos()
            * ((&e_lng_r - s_lng_r)/2.).sin().powi(2)
        };

        return d.sqrt().asin() * 2. * RADIUS;
    }
}

#[duplicate_item(
    VectorType                      Generics;
    [ f64 ]                         [];
    [ &F64Array1 ]                  [];
    [ &F64ArcArray1 ]               [];
    [ &F64ArrayView<'a, Ix1> ]      [ 'a ];
    [ &F64ArrayViewMut<'a, Ix1> ]   [ 'a ];
)]
impl<Generics> OffsetByVector<VectorType> for Haversine {
    fn offset(
        s:&dyn LatLngArray,
        distance:VectorType,
        bearing:VectorType,
    ) -> F64LatLngArray {
        let bearing_r = bearing / 180. * PI;

        let ang_dist = distance / RADIUS;
        let s_latlng_r= s.to_rad();
        let (s_lat_r, s_lng_r) = (
            s_latlng_r.column(0), s_latlng_r.column(1)
        );

        let e_lat_r = {
            (
                s_lat_r.sin()*ang_dist.cos()
                + s_lat_r.cos()*ang_dist.sin()*bearing_r.cos()
            ).asin()
        };

        let e_lng_r = {
            &s_lng_r + (
                bearing_r.sin()*ang_dist.sin()*s_lat_r.cos()
            ).atan2(
                ang_dist.cos()-s_lat_r.sin()*e_lat_r.sin()
            )
        };

        // Create our empty 2 dimensional array with NO COLUMNS to contain the results;
        let mut e_latlng_r = {
            F64LatLngArray::zeros((e_lng_r.len(), 0))
        };

        // Push the data through.
        e_latlng_r.push_column(e_lat_r.view()).unwrap();
        e_latlng_r.push_column(e_lng_r.view()).unwrap();

        e_latlng_r = e_latlng_r.to_dec();
        e_latlng_r.normalize();

        return e_latlng_r;
    }
}

#[duplicate_item(
    VectorType                      Generics;
    [ f64 ]                         [];
    [ &F64Array1 ]                  [];
    [ &F64ArcArray1 ]               [];
    [ &F64ArrayView<'a, Ix1> ]      [ 'a ];
    [ &F64ArrayViewMut<'a, Ix1> ]   [ 'a ];
)]
impl<Generics> CheckDistance<VectorType> for Haversine {
    fn within_distance(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:VectorType,
    ) -> BoolArray1 {
        return (Self::distance(s, e) - distance).le(&0.);
    }
}
