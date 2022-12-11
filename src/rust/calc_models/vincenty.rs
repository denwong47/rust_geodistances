use std::f64::consts::PI;

use ndarray::{
    Array1,
};
use ndarray_numeric::{
    F64Array,
    F64Array1,
    F64LatLngArray,

    ArrayWithF64Methods,
    ArrayWithF64AngularMethods,
    ArrayWithF64LatLngMethods,
    ArrayWithF64PartialOrd,
};

use super::traits::{
    LatLng,
    LatLngArray,
    CalculateDistance,
    CheckDistance,
    OffsetByVector
};
use super::config::{
    EPS,
    ELLIPSE_WGS84_A,
    ELLIPSE_WGS84_B,
    ELLIPSE_WGS84_F,
};

pub const ITERATIONS:u16 = 1000;

///  Vincenty solutions of geodescis on the ellipsoid
///  Adapted from https://www.movable-type.co.uk/scripts/latlong-vincenty.html
///
///  Geodesics on Ellipsoid calculation - accurate but iterative
///
///  Discard calculation if latitude doesn't make any sense
///  This algorithm does not work when Latitude = 90 or -90
pub struct Vincenty;
impl CalculateDistance for Vincenty {
    #[allow(non_snake_case)]
    fn distance(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
    ) -> F64Array1 {
        let eps:f64 = (2 as f64).powi(-52);

        // Discard calculation if latitude doesn't make any sense
        let (s_lat, s_lng) = (s[0], s[1]);
        let (s_lat_r, s_lng_r) = (s_lat * PI / 180., s_lng * PI /180.);

        let e_latlng_r = e.to_rad();
        let (e_lat_r, e_lng_r) = (e_latlng_r.column(0), e_latlng_r.column(1));

        let diff_lng_r = &e_lng_r - s_lng_r;
        let tan_reduced_s_lat_r = (1.-ELLIPSE_WGS84_F) * s_lat_r.tan();
        let cos_reduced_s_lat_r = (tan_reduced_s_lat_r.powi(2) + 1.).sqrt().powi(-1);
        let sin_reduced_s_lat_r = tan_reduced_s_lat_r * cos_reduced_s_lat_r;

        let tan_reduced_e_lat_r = (1.-ELLIPSE_WGS84_F) * e_lat_r.tan();
        let cos_reduced_e_lat_r = (tan_reduced_e_lat_r.powi(2) + 1.).sqrt().powi(-1);
        let sin_reduced_e_lat_r = tan_reduced_e_lat_r * cos_reduced_e_lat_r;

        let mut _lambda:F64Array1 = diff_lng_r;
        let mut _lambda_dash:F64Array1 = _lambda*0.;

        let mut sin_lng_r:F64Array1 = _lambda*0.;
        let mut cos_lng_r:F64Array1 = _lambda*0.;

        // Does ndarray does not implement ||
        // but BitOr on bool is the same as || so | is correct here.
        let antipodal:Array1<bool> = (diff_lng_r.gt(&(PI/2.))) | ((&e_lat_r - s_lat_r).abs().gt(&(PI/2.)));

        let mut ang_dist:F64Array1 = antipodal.map(|b| if *b {PI} else {0.});
        let mut sin_ang_dist:F64Array1 = ang_dist*0.;
        let mut cos_ang_dist:F64Array1 = antipodal.map(|b| if *b {-1.} else {1.});

        let mut sin_azimuth_of_geodesic_at_equator:F64Array1 = cos_reduced_e_lat_r*0.;
        let mut cos_2_ang_dist_from_equator_bisect:F64Array1 = cos_reduced_e_lat_r*0.;
        let mut cos_sq_azimuth_of_geodesic_at_equator:F64Array1 = cos_reduced_e_lat_r*0.;

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

        for _ in 0..ITERATIONS {
            sin_lng_r = _lambda.sin();
            cos_lng_r = _lambda.cos();

            let sin_sq_ang_dist = {
                (cos_reduced_e_lat_r*sin_lng_r).powi(2)
                + (
                    cos_reduced_s_lat_r*sin_reduced_e_lat_r
                    - sin_reduced_s_lat_r*cos_reduced_e_lat_r*cos_lng_r
                ).powi(2)
            };

            // Start vectorizing here?
            // _lambda.mapv_inplace
            if sin_sq_ang_dist.abs().lt(1e-24) { break }

            sin_ang_dist = sin_sq_ang_dist.sqrt();
            cos_ang_dist = {
                sin_reduced_e_lat_r*sin_reduced_s_lat_r
                + cos_reduced_e_lat_r*cos_lng_r*cos_reduced_s_lat_r
            };

            ang_dist = sin_ang_dist.atan2_arr(cos_ang_dist);

            sin_azimuth_of_geodesic_at_equator = {
                cos_reduced_e_lat_r*sin_lng_r*cos_reduced_s_lat_r/sin_ang_dist
            };

            cos_sq_azimuth_of_geodesic_at_equator = {
                1. - sin_azimuth_of_geodesic_at_equator.powi(2)
            };

            cos_2_ang_dist_from_equator_bisect = {
                if cos_sq_azimuth_of_geodesic_at_equator.abs() > eps {
                    cos_ang_dist - 2.*sin_reduced_s_lat_r*sin_reduced_e_lat_r/cos_sq_azimuth_of_geodesic_at_equator
                } else {
                    0.
                }
            };

            let _c = {
                ELLIPSE_WGS84_F / 16.
                * cos_sq_azimuth_of_geodesic_at_equator
                * (4.+ELLIPSE_WGS84_F*(4.-3.*cos_sq_azimuth_of_geodesic_at_equator))
            };

            _lambda_dash = _lambda;

            _lambda = {
                diff_lng_r + (1.-_c) * ELLIPSE_WGS84_F
                * sin_azimuth_of_geodesic_at_equator
                * (ang_dist + _c*sin_ang_dist*(
                    cos_2_ang_dist_from_equator_bisect
                    +_c*cos_ang_dist*(
                        -1.
                        +2.*cos_2_ang_dist_from_equator_bisect.powi(2)
                    )
                ))
            };

            if (_lambda-_lambda_dash).abs() <= EPS { break }
        }

        let _uSq = cos_sq_azimuth_of_geodesic_at_equator * (
            ELLIPSE_WGS84_A.powi(2) - ELLIPSE_WGS84_B.powi(2)
        ) / ELLIPSE_WGS84_B.powi(2);

        let _a = 1.+_uSq/16384.*(4096.+_uSq*(-768.+_uSq*(320.-175.*_uSq)));
        let _b = _uSq/1024. * (256.+_uSq*(-128.+_uSq*(74.-47.*_uSq)));

        let delta_ang_dist = {
            _b*sin_ang_dist*(
                cos_2_ang_dist_from_equator_bisect
                + _b/4.*(
                    cos_ang_dist*(
                        -1.+2.*cos_2_ang_dist_from_equator_bisect.powi(2)
                    )
                    - _b/6.*cos_2_ang_dist_from_equator_bisect*(
                        -3.+4.*sin_ang_dist.powi(2)
                    )*(
                        -3.+4.*cos_2_ang_dist_from_equator_bisect.powi(2)
                    )
                )
            )
        };

        return (ang_dist-delta_ang_dist)*ELLIPSE_WGS84_B*_a
    }
}
impl CheckDistance for Vincenty {
    fn within_distance(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:f64,
    ) -> Array1<bool> {
        return Self::distance(s, e).le(&distance);
    }
}
impl OffsetByVector for Vincenty {
    #[allow(non_snake_case)]
    fn offset(
        s:&dyn LatLngArray,
        distance:f64,
        bearing:f64,
    ) -> F64LatLngArray {
        let bearing_r = bearing / 180. * PI;

        let (s_lat_r, s_lng_r) = s.as_rad();

        let sin_bearing_r = bearing_r.sin();
        let cos_bearing_r = bearing_r.cos();

        let tan_u1 = (1.-ELLIPSE_WGS84_F) * s_lat_r.tan();
        let cos_u1 = 1. / ((1. + tan_u1.powi(2))).sqrt();
        let sin_u1 = tan_u1 * cos_u1;

        let ang_dist_on_sphere_from_equator = (tan_u1).atan2(cos_bearing_r); // ang_dist_on_sphere_from_equator = angular distance on the sphere from the equator to P1
        let sin_azimuth_of_geodesic_at_equator = cos_u1 * sin_bearing_r;          // α = azimuth of the geodesic at the equator
        let cos_sq_azimuth_of_geodesic_at_equator = 1. - sin_azimuth_of_geodesic_at_equator.powi(2);
        let _uSq = cos_sq_azimuth_of_geodesic_at_equator * (ELLIPSE_WGS84_A.powi(2) - ELLIPSE_WGS84_B.powi(2)) / (ELLIPSE_WGS84_B.powi(2));
        let _a = 1. + _uSq/16384.*(4096.+_uSq*(-768.+_uSq*(320.-175.*_uSq)));
        let _b = _uSq/1024. * (256.+_uSq*(-128.+_uSq*(74.-47.*_uSq)));

        let mut sin_ang_dist = 0.;
        let mut cos_ang_dist = 0.; // ang_dist = angular distance P₁ P₂ on the sphere
        let mut cos_2_ang_dist_from_equator_bisect = 0.; // σₘ = angular distance on the sphere from the equator to the midpoint of the line

        drop(&sin_ang_dist);
        drop(&cos_ang_dist);
        drop(&cos_2_ang_dist_from_equator_bisect);

        let mut ang_dist = distance / (ELLIPSE_WGS84_B*_a);
        let mut ang_dist_dash = 0.;
        let mut delta_ang_dist = 0.;

        drop(&ang_dist_dash);
        drop(&delta_ang_dist);

        for iteration in 0..ITERATIONS {
            cos_2_ang_dist_from_equator_bisect = (2.*ang_dist_on_sphere_from_equator + ang_dist).cos();
            sin_ang_dist = ang_dist.sin();
            cos_ang_dist = ang_dist.cos();
            delta_ang_dist = {
                _b*sin_ang_dist*(
                    cos_2_ang_dist_from_equator_bisect
                    + _b/4.*(
                        cos_ang_dist*(
                            -1.+2.*cos_2_ang_dist_from_equator_bisect.powi(2)
                        )
                        - _b/6.*cos_2_ang_dist_from_equator_bisect*(
                            -3.+4.*sin_ang_dist.powi(2)
                        )*(
                            -3.+4.*cos_2_ang_dist_from_equator_bisect.powi(2)
                        )
                    )
                )
            };
            ang_dist_dash = ang_dist;
            ang_dist = distance / (ELLIPSE_WGS84_B*_a) + delta_ang_dist;

            if (ang_dist-ang_dist_dash).abs() <= EPS { break }

            if iteration>ITERATIONS-1 {
                // Vincenty failed to converge
                return None
            }
        }

        let _x = sin_u1*sin_ang_dist - cos_u1*cos_ang_dist*cos_bearing_r;
        let e_lat_r = {
            (
                sin_u1*cos_ang_dist
                + cos_u1*sin_ang_dist*cos_bearing_r
            ).atan2(
                (
                    1.-ELLIPSE_WGS84_F
                )*(
                    sin_azimuth_of_geodesic_at_equator.powi(2)
                    + _x.powi(2)
                ).sqrt()
            )
        };
        let _lambda = (sin_ang_dist*sin_bearing_r).atan2(cos_u1*cos_ang_dist - sin_u1*sin_ang_dist*cos_bearing_r);
        let _c = {
            ELLIPSE_WGS84_F / 16.
            *cos_sq_azimuth_of_geodesic_at_equator
            *(
                4.+ELLIPSE_WGS84_F*(4.-3.*cos_sq_azimuth_of_geodesic_at_equator)
            )
        };
        let e_lng_r = {
            s_lng_r
            + _lambda
            - (
                (1.-_c)
                * ELLIPSE_WGS84_F
                * sin_azimuth_of_geodesic_at_equator
                * (
                    ang_dist
                    + _c*sin_ang_dist*(
                        cos_2_ang_dist_from_equator_bisect
                        +_c*cos_ang_dist*(
                            -1.
                            +2.*cos_2_ang_dist_from_equator_bisect.powi(2)
                        )
                    )
                )
            )
        };

        return Some(LatLng::new_from_rad(
            e_lat_r,
            e_lng_r,
        ));
    }
}
