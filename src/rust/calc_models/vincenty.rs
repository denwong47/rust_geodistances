use std::f64::consts::PI;

use duplicate::{
    duplicate_item
};

use ndarray::{
    Axis,
    Ix1,
    Zip,
};
use rayon::prelude::*;

use ndarray_numeric::{
    ArrayWithF64Methods,
    ArrayWithF64Atan2Methods,
    ArrayWithF64PartialOrd,
    ArrayWithF64AngularMethods,
    ArrayWithF64LatLngMethods,

    BoolArray1,

    F64Array,
    F64Array1,
    F64Array2,
    F64ArcArray1,
    F64ArrayView,
    F64ArrayViewMut,
    // F64LatLng,
    F64LatLngArray,

    ArrayWithBoolIterMethods,
};

use super::{
    config,
};

use super::traits::{
    LatLng,
    LatLngArray,
    CalculateDistance,
    OffsetByVector
};

///  Vincenty solutions of geodescis on the ellipsoid
///  Adapted from https://www.movable-type.co.uk/scripts/latlong-vincenty.html
///
///  Geodesics on Ellipsoid calculation - accurate but iterative
///
///  Discard calculation if latitude doesn't make any sense
///  This algorithm does not work when Latitude = 90 or -90
pub struct Vincenty;
impl CalculateDistance for Vincenty {
    /// Internal function
    #[allow(non_snake_case)]
    fn distance_from_point_rad(
        s_lat_r:&f64,
        s_lng_r:&f64,
        e_lat_r:&F64ArrayView<'_, Ix1>,
        e_lng_r:&F64ArrayView<'_, Ix1>,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1 {
        let settings_default = &config::CalculationSettings::default();

        let eps:f64 = settings.unwrap_or(&settings_default).eps;
        let tolerance:f64 = settings.unwrap_or(&settings_default).tolerance;
        let ellipse_a:f64 = settings.unwrap_or(&settings_default).ellipse_a;
        let ellipse_b:f64 = settings.unwrap_or(&settings_default).ellipse_b;
        let ellipse_f:f64 = settings.unwrap_or(&settings_default).ellipse_f;
        let max_iterations:usize = settings.unwrap_or(&settings_default).max_iterations;

        assert!(tolerance>0., "`tolerance` must be positive, yet {:?} provided.", tolerance);

        let diff_lng_r = e_lng_r - *s_lng_r;
        let tan_reduced_s_lat_r = (1.-ellipse_f) * s_lat_r.tan();
        let cos_reduced_s_lat_r = (tan_reduced_s_lat_r.powi(2) + 1.).sqrt().powi(-1);
        let sin_reduced_s_lat_r = tan_reduced_s_lat_r * cos_reduced_s_lat_r;

        let tan_reduced_e_lat_r = (1.-ellipse_f) * e_lat_r.tan();
        let cos_reduced_e_lat_r = (tan_reduced_e_lat_r.powi(2) + 1.).sqrt().powi(-1);
        let sin_reduced_e_lat_r = tan_reduced_e_lat_r * &cos_reduced_e_lat_r;

        let shape = (diff_lng_r.len(), );
        let mut lambda:F64Array1 = diff_lng_r.clone();
        let mut lambda_dash:F64Array1 = F64Array1::zeros(shape);

        let mut sin_lng_r:F64Array1 = F64Array1::zeros(shape);
        let mut cos_lng_r:F64Array1 = F64Array1::zeros(shape);

        // Does ndarray does not implement ||
        // but BitOr on bool is the same as || so | is correct here.
        let antipodal:BoolArray1 = (diff_lng_r.gt(&(PI/2.))) | ((e_lat_r - *s_lat_r).abs().gt(&(PI/2.)));

        let mut ang_dist:F64Array1 = antipodal.map(|b| if *b {PI} else {0.});
        let mut sin_ang_dist:F64Array1 = F64Array1::zeros(shape);
        let mut cos_ang_dist:F64Array1 = antipodal.map(|b| if *b {-1.} else {1.});

        let mut sin_azimuth_of_geodesic_at_equator:F64Array1 = F64Array1::zeros(shape);
        let mut cos_2_ang_dist_from_equator_bisect:F64Array1 = F64Array1::zeros(shape);
        let mut cos_sq_azimuth_of_geodesic_at_equator:F64Array1 = F64Array1::zeros(shape);

        // Dropping a useless reference.
        // Just to get around the compiler "value never used" check.
        drop(&sin_lng_r);
        drop(&cos_lng_r);
        drop(&sin_azimuth_of_geodesic_at_equator);

        // TODO vectorize this check
        // e_lat, e_lng does not exist yet
        // if (-e_lat+s_lat).abs() <= eps && (-e_lng+s_lng).abs() <= eps {
        //     return Some(0.)
        // }

        for _i in 0..max_iterations {
            sin_lng_r = lambda.sin();
            cos_lng_r = lambda.cos();

            let sin_sq_ang_dist = {
                (&cos_reduced_e_lat_r*&sin_lng_r).powi(2)
                + (
                    cos_reduced_s_lat_r*&sin_reduced_e_lat_r
                    - sin_reduced_s_lat_r*&cos_reduced_e_lat_r*&cos_lng_r
                ).powi(2)
            };

            // let next_iteration_mask = sin_sq_ang_dist.abs().gt(&tolerance);
            let next_iteration_mask = (&lambda_dash-&lambda).abs().gt(&tolerance);

            // If nothing needs to be iterated any more, stop the for loop
            // println!("{}, {}, {}", _i, sin_sq_ang_dist.abs(), next_iteration_mask.any());
            if !next_iteration_mask.any() { break }

            // Mapping block
            {
                lambda.indexed_iter_mut()
                       .map(
                        |(idx, _lambda)| {
                            // Skip iteration if we are already within tolerance
                            if next_iteration_mask[idx] {
                                // Get all variables local to idx
                                let _sin_lng_r = sin_lng_r[idx];
                                let _cos_lng_r = cos_lng_r[idx];

                                let _sin_sq_ang_dist = sin_sq_ang_dist[idx];

                                let _sin_reduced_e_lat_r = sin_reduced_e_lat_r[idx];
                                let _cos_reduced_e_lat_r = cos_reduced_e_lat_r[idx];

                                let mut _sin_ang_dist = sin_ang_dist.get_mut(idx).unwrap();
                                let mut _cos_ang_dist = cos_ang_dist.get_mut(idx).unwrap();
                                *_sin_ang_dist = _sin_sq_ang_dist.sqrt();
                                *_cos_ang_dist = {
                                    _sin_reduced_e_lat_r*sin_reduced_s_lat_r
                                    + _cos_reduced_e_lat_r*_cos_lng_r*cos_reduced_s_lat_r
                                };

                                let mut _ang_dist = ang_dist.get_mut(idx).unwrap();
                                *_ang_dist = _sin_ang_dist.atan2(*_cos_ang_dist);

                                let mut _sin_azimuth_of_geodesic_at_equator = sin_azimuth_of_geodesic_at_equator.get_mut(idx).unwrap();
                                *_sin_azimuth_of_geodesic_at_equator = {
                                    _cos_reduced_e_lat_r*_sin_lng_r*cos_reduced_s_lat_r/(*_sin_ang_dist)
                                };

                                let mut _cos_sq_azimuth_of_geodesic_at_equator = cos_sq_azimuth_of_geodesic_at_equator.get_mut(idx).unwrap();
                                *_cos_sq_azimuth_of_geodesic_at_equator = {
                                    1. - _sin_azimuth_of_geodesic_at_equator.powi(2)
                                };

                                let mut _cos_2_ang_dist_from_equator_bisect = cos_2_ang_dist_from_equator_bisect.get_mut(idx).unwrap();
                                *_cos_2_ang_dist_from_equator_bisect = {
                                    if _cos_sq_azimuth_of_geodesic_at_equator.abs() > eps {
                                        *_cos_ang_dist - 2.*sin_reduced_s_lat_r*_sin_reduced_e_lat_r/((*_cos_sq_azimuth_of_geodesic_at_equator))
                                    } else {
                                        0.
                                    }
                                };

                                let _c = {
                                    ellipse_f / 16.
                                    * (*_cos_sq_azimuth_of_geodesic_at_equator)
                                    * (4.+ellipse_f*(4.-3.*(*_cos_sq_azimuth_of_geodesic_at_equator)))
                                };

                                let mut _lambda_dash = lambda_dash.get_mut(idx).unwrap();
                                *_lambda_dash = _lambda.clone();

                                *_lambda = {
                                    diff_lng_r[idx] + (1.-_c) * ellipse_f
                                    * (*_sin_azimuth_of_geodesic_at_equator)
                                    * (*_ang_dist + _c* (*_sin_ang_dist)*(
                                        *_cos_2_ang_dist_from_equator_bisect
                                        +_c*(*_cos_ang_dist)*(
                                            -1.
                                            +2.*(*_cos_2_ang_dist_from_equator_bisect).powi(2)
                                        )
                                    ))
                                };
                            }
                        }
                       )
                       .count();
            }
        }

        let uSq = cos_sq_azimuth_of_geodesic_at_equator * (
            ellipse_a.powi(2) - ellipse_b.powi(2)
        ) / ellipse_b.powi(2);

        let _a = 1.+&uSq/16384.*(4096.+&uSq*(-768.+&uSq*(320.-175.*&uSq)));
        let _b = &uSq/1024. * (256.+&uSq*(-128.+&uSq*(74.-47.*&uSq)));

        let delta_ang_dist = {
            &_b*&sin_ang_dist*(
                &cos_2_ang_dist_from_equator_bisect
                + &_b/4.*(
                    &cos_ang_dist*(
                        -1.+2.*&cos_2_ang_dist_from_equator_bisect.powi(2)
                    )
                    - &_b/6.*&cos_2_ang_dist_from_equator_bisect*(
                        -3.+4.*sin_ang_dist.powi(2)
                    )*(
                        -3.+4.*cos_2_ang_dist_from_equator_bisect.powi(2)
                    )
                )
            )
        };

        return (ang_dist-delta_ang_dist)*_a
    }

    fn distance_rad(
        s_lat_r:&F64ArrayView<'_, Ix1>,
        s_lng_r:&F64ArrayView<'_, Ix1>,
        e_lat_r:&F64ArrayView<'_, Ix1>,
        e_lng_r:&F64ArrayView<'_, Ix1>,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2 {
        let mut results = F64Array2::zeros((0, e_lat_r.len()));

        // COPIED FROM HAVERSINE - CHANGE
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

        let ellipse_b: f64 = settings.unwrap_or(
            &config::CalculationSettings::default()
        ).ellipse_b;

        let d = Self::distance_from_point_rad(&s_lat_r, &s_lng_r, &e_lat_r, &e_lng_r, settings,);

        return d * ellipse_b;
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

        let ellipse_b: f64 = settings.unwrap_or(
            &config::CalculationSettings::default()
        ).ellipse_b;

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
        } * ellipse_b;

        return results;
    }

}

#[duplicate_item(
    // Arrays
    [
        __vector_type__      [ &F64Array1 ]
        __impl_generics__    []
    ]
    [
        __vector_type__      [ &F64ArcArray1 ]
        __impl_generics__    []
    ]
    [
        __vector_type__      [ &F64ArrayView<'a, Ix1> ]
        __impl_generics__    [ 'a ]
    ]
    [
        __vector_type__      [ &F64ArrayViewMut<'a, Ix1> ]
        __impl_generics__    [ 'a ]
    ]
)]
/// Array implementation
impl<__impl_generics__> OffsetByVector<__vector_type__> for Vincenty {
    #[allow(non_snake_case)]
    fn displace(
        s:&dyn LatLngArray,
        distance:__vector_type__,
        bearing:__vector_type__,
        settings: Option<&config::CalculationSettings>,
    ) -> F64LatLngArray {
        let settings_default = &config::CalculationSettings::default();

        // let eps:f64 = settings.unwrap_or(&settings_default).eps;
        let tolerance:f64 = settings.unwrap_or(&settings_default).tolerance;
        let ellipse_a:f64 = settings.unwrap_or(&settings_default).ellipse_a;
        let ellipse_b:f64 = settings.unwrap_or(&settings_default).ellipse_b;
        let ellipse_f:f64 = settings.unwrap_or(&settings_default).ellipse_f;
        let max_iterations:usize = settings.unwrap_or(&settings_default).max_iterations;

        let bearing_r = bearing / 180. * PI;

        let s_latlng_r= s.to_rad();
        let (s_lat_r, s_lng_r) = (
            s_latlng_r.column(0), s_latlng_r.column(1)
        );

        let shape = (s_latlng_r.shape()[0], );

        let sin_bearing_r = bearing_r.sin();
        let cos_bearing_r = bearing_r.cos();

        let tan_u1 = s_lat_r.tan() * (1.-ellipse_f);
        let cos_u1 = 1. / ((1. + tan_u1.powi(2))).sqrt();
        let sin_u1 = &tan_u1 * &cos_u1;

        let ang_dist_on_sphere_from_equator = tan_u1.atan2(cos_bearing_r.view()); // ang_dist_on_sphere_from_equator = angular distance on the sphere from the equator to P1
        let sin_azimuth_of_geodesic_at_equator = &cos_u1 * &sin_bearing_r;          // α = azimuth of the geodesic at the equator
        let cos_sq_azimuth_of_geodesic_at_equator = 1. - sin_azimuth_of_geodesic_at_equator.powi(2);
        let uSq = &cos_sq_azimuth_of_geodesic_at_equator * (ellipse_a.powi(2) - ellipse_b.powi(2)) / (ellipse_b.powi(2));
        let _a = 1. + &uSq/16384.*(4096.+&uSq*(-768.+&uSq*(320.-175.*&uSq)));
        let _b = &uSq/1024. * (256.+&uSq*(-128.+&uSq*(74.-47.*&uSq)));

        let mut sin_ang_dist = F64Array1::zeros(shape);
        let mut cos_ang_dist = F64Array1::zeros(shape); // ang_dist = angular distance P₁ P₂ on the sphere
        let mut cos_2_ang_dist_from_equator_bisect = F64Array1::zeros(shape); // σₘ = angular distance on the sphere from the equator to the midpoint of the line

        drop(&sin_ang_dist);
        drop(&cos_ang_dist);
        drop(&cos_2_ang_dist_from_equator_bisect);

        let mut ang_dist = distance / (ellipse_b*&_a);
        let mut ang_dist_dash = F64Array1::zeros(shape);

        drop(&ang_dist_dash);

        for _ in 0..max_iterations {
            cos_2_ang_dist_from_equator_bisect = (2.*&ang_dist_on_sphere_from_equator + &ang_dist).cos();
            sin_ang_dist = ang_dist.sin();
            cos_ang_dist = ang_dist.cos();

            let next_iteration_mask = (&ang_dist-ang_dist_dash).abs().gt(&tolerance);

            // If nothing needs to be iterated any more, stop the for loop
            if !next_iteration_mask.any() { break }

            ang_dist_dash = ang_dist.clone();

            {
                ang_dist.indexed_iter_mut()
                        .map(
                            |(idx, _ang_dist)| {
                                if next_iteration_mask[idx] {
                                    let _delta_ang_dist = _b[idx]*sin_ang_dist[idx]*(
                                        cos_2_ang_dist_from_equator_bisect[idx]
                                        + _b[idx]/4.*(
                                            cos_ang_dist[idx]*(
                                                -1.+2.*cos_2_ang_dist_from_equator_bisect[idx].powi(2)
                                            )
                                            - _b[idx]/6.*cos_2_ang_dist_from_equator_bisect[idx]*(
                                                -3.+4.*sin_ang_dist[idx].powi(2)
                                            )*(
                                                -3.+4.*cos_2_ang_dist_from_equator_bisect[idx].powi(2)
                                            )
                                        )
                                    );

                                    *_ang_dist = distance[idx] / (ellipse_b*_a[idx]) + _delta_ang_dist;
                                }
                            }
                        )
                        .count()
            };
        }

        let _x = &sin_u1*&sin_ang_dist - &cos_u1*&cos_ang_dist*&cos_bearing_r;
        let e_lat_r = {
            (
                &sin_u1*&cos_ang_dist
                + &cos_u1*&sin_ang_dist*&cos_bearing_r
            ).atan2(
                (
                    1.-ellipse_f
                )*(
                    &sin_azimuth_of_geodesic_at_equator.powi(2)
                    + &_x.powi(2)
                ).sqrt()
            )
        };
        let _lambda = (&sin_ang_dist*&sin_bearing_r).atan2(&cos_u1*&cos_ang_dist - &sin_u1*&sin_ang_dist*&cos_bearing_r);
        let _c = {
            ellipse_f / 16.
            *&cos_sq_azimuth_of_geodesic_at_equator
            *(
                4.+ellipse_f*(4.-3.*&cos_sq_azimuth_of_geodesic_at_equator)
            )
        };
        let e_lng_r = {
            &s_lng_r
            + &_lambda
            - (
                (1.-&_c)
                * ellipse_f
                * &sin_azimuth_of_geodesic_at_equator
                * (
                    &ang_dist
                    + &_c*&sin_ang_dist*(
                        &cos_2_ang_dist_from_equator_bisect
                        +&_c*&cos_ang_dist*(
                            -1.
                            +2.*&cos_2_ang_dist_from_equator_bisect.powi(2)
                        )
                    )
                )
            )
        };

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


/// Scalar implementation
///
/// This just call the Array implementation of the same thing.
impl OffsetByVector<f64> for Vincenty {
    #[allow(non_snake_case)]
    fn displace(
        s:&dyn LatLngArray,
        distance:f64,
        bearing:f64,
        settings: Option<&config::CalculationSettings>,
    ) -> F64LatLngArray {
        let shape = (s.shape()[0],);
        let distance_arr = F64Array::from_elem(shape, distance);
        let bearing_arr = F64Array::from_elem(shape, bearing);

        return Self::displace(
            s,
            &distance_arr, &bearing_arr,
            settings
        );
    }
}
