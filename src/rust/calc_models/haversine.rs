use std::f64::consts::PI;

use duplicate::duplicate_item;

use ndarray::{
    Axis,
    Ix1,
    // Ix2,
    // NdIndex,
    // s,
    Zip,
};
use rayon::prelude::*;

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
    BoolArray2,

    F64Array1,
    F64Array2,
    F64ArcArray1,
    F64ArrayView,
    F64ArrayViewMut,
    // F64LatLng,
    F64LatLngArray,

    SquareShapedArray,
};

use super::config;

/// Haversine calculation
/// Assumes spherical world - fast but has errors up to ~0.35%
/// Adapted from https://towardsdatascience.com/better-parallelization-with-numba-3a41ca69452e
pub struct Haversine;
impl CalculateDistance for Haversine {
    /// Internal function
    fn distance_from_point_rad(
        s_lat_r:&f64,
        s_lng_r:&f64,
        e_lat_r:&F64ArrayView<'_, Ix1>,
        e_lng_r:&F64ArrayView<'_, Ix1>,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1 {
        drop(&settings);

        return {
            // bear in mind that e_lat_r is an array while s_lat_r is f64
            ((e_lat_r - *s_lat_r)/2.).sin().powi(2)
            + s_lat_r.cos()*e_lat_r.cos()
            * ((e_lng_r - *s_lng_r)/2.).sin().powi(2)
        }.sqrt().asin() * 2.;
    }

    fn distance_rad(
        s_lat_r:&F64ArrayView<'_, Ix1>,
        s_lng_r:&F64ArrayView<'_, Ix1>,
        e_lat_r:&F64ArrayView<'_, Ix1>,
        e_lng_r:&F64ArrayView<'_, Ix1>,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2 {
        let mut results = F64Array2::zeros((0, e_lat_r.len()));

        Zip::from(s_lat_r)
            .and(s_lng_r)
            .for_each(|lat, lng| {
                results.push_row(
                    Self::distance_from_point_rad(
                        &lat, &lng,
                        e_lat_r, e_lng_r,
                        settings,
                    ).view()
                ).unwrap();
            });

        return results;
    }

    fn distance_from_point(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1 {

        let (s_lat, s_lng) = (s[0], s[1]);

        let (s_lat_r, s_lng_r) = (s_lat * PI / 180., s_lng * PI /180.);

        let e_latlng_r = e.to_rad();
        let (e_lat_r, e_lng_r) = (e_latlng_r.column(0), e_latlng_r.column(1));

        let radius: f64 = settings.unwrap_or(
            &config::CalculationSettings::default()
        ).spherical_radius;

        let d = Self::distance_from_point_rad(&s_lat_r, &s_lng_r, &e_lat_r, &e_lng_r, settings,);

        return d * radius;
    }

    fn distance(
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2 {
        let (s_latlng_r, e_latlng_r) = (s.to_rad(), e.to_rad());
        let (e_lat_r, e_lng_r) = (e_latlng_r.column(0), e_latlng_r.column(1));

        let shape = (s.shape()[0], e.shape()[0]);

        let workers: usize = settings.unwrap_or(
            &config::CalculationSettings::default()
        ).workers;
        let chunk_size: usize = (shape.0 as f32 / workers as f32).ceil() as usize;

        let radius: f64 = settings.unwrap_or(
            &config::CalculationSettings::default()
        ).spherical_radius;

        let results = {
            s_latlng_r.axis_chunks_iter(Axis(0), chunk_size)
                     .into_par_iter()
                     .map(| s_latlng_r_chunk | {
                            let (s_lat_r, s_lng_r) = (s_latlng_r_chunk.column(0), s_latlng_r_chunk.column(1));

                            Self::distance_rad(
                                &s_lat_r, &s_lng_r,
                                &e_lat_r, &e_lng_r,
                                settings,
                            )
                        }
                     )
                     .reduce(
                        move || F64Array2::zeros((0, shape.1)),
                        | mut a, b | {
                            a.append(Axis(0), b.view()).unwrap();
                            return a;
                        }
                     )
        } * radius;

        return results;
    }

    fn distance_within_array(
        s:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2 {
        let s_owned = s.to_owned();

        let workers: usize = settings.unwrap_or(
            &config::CalculationSettings::default()
        ).workers;

        return F64Array2::from_mapped_array2_fn(
            &s_owned.view(),
            | s, e | {
                println!("sn={:?} en={:?}", &s, &e);
                Self::distance_from_point(&s, &e.to_owned(), settings)
            },
            workers,
            Some(true),
        );
    }
}

#[duplicate_item(
    __vector_type__                 __impl_generics__;
    [ f64 ]                         [];
    [ &F64Array1 ]                  [];
    [ &F64ArcArray1 ]               [];
    [ &F64ArrayView<'a, Ix1> ]      [ 'a ];
    [ &F64ArrayViewMut<'a, Ix1> ]   [ 'a ];
)]
impl<__impl_generics__> OffsetByVector<__vector_type__> for Haversine {
    fn offset(
        s:&dyn LatLngArray,
        distance:__vector_type__,
        bearing:__vector_type__,
        settings: Option<&config::CalculationSettings>,
    ) -> F64LatLngArray {
        let bearing_r = bearing / 180. * PI;
        let radius: f64 = settings.unwrap_or(
            &config::CalculationSettings::default()
        ).spherical_radius;

        let ang_dist = distance / radius;
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
    __vector_type__                 __impl_generics__;
    [ f64 ]                         [];
    [ &F64Array1 ]                  [];
    [ &F64ArcArray1 ]               [];
    [ &F64ArrayView<'a, Ix1> ]      [ 'a ];
    [ &F64ArrayViewMut<'a, Ix1> ]   [ 'a ];
)]
impl<__impl_generics__> CheckDistance<__vector_type__> for Haversine {
    fn within_distance_of_point(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:__vector_type__,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray1 {
        return (Self::distance_from_point(s, e, settings,) - distance).le(&0.);
    }

    fn within_distance(
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: __vector_type__,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray2 {
        return (Self::distance(s, e, settings,) - distance).le(&0.);
    }

    fn within_distance_among_array(
            s:&dyn LatLngArray,
            distance: __vector_type__,
            settings: Option<&config::CalculationSettings>,
        ) -> BoolArray2 {
        return (Self::distance_within_array(s, settings) - distance).le(&0.);
    }
}
