use std::f64::consts::PI;

use crate::data::structs::LatLng;

use crate::geodistances::traits::{CalculateDistance, CheckDistance, OffsetByVector};

const ELLIPSE_WGS84_A:f64 = 6378.137;
const ELLIPSE_WGS84_B:f64 = 6356.752314245;
const ELLIPSE_WGS84_F:f64 = 1./298.257223563;

const ITERATIONS:u16 = 1000;

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
        s:&LatLng,
        e:&LatLng,
    )->Option<f64> {
        let eps:f64 = (2 as f64).powi(-52);

        // Discard calculation if latitude doesn't make any sense
        if -90. < s.lat && s.lat < 90. && -90. < e.lat && e.lat < 90. {
            if (s.lat-e.lat).abs() <= eps && (s.lng-e.lng).abs() <= eps {
                return Some(0.)
            }

            let s_lat_r = s.lat * PI / 180.;
            let s_lng_r = s.lng * PI / 180.;
            let e_lat_r = e.lat * PI / 180.;
            let e_lng_r = e.lng * PI / 180.;

            let diff_lng_r = e_lng_r - s_lng_r;
            let tan_reduced_s_lat_r = (1.-ELLIPSE_WGS84_F) * s_lat_r.tan();
            let cos_reduced_s_lat_r = 1. / (1.+tan_reduced_s_lat_r.powi(2)).sqrt();
            let sin_reduced_s_lat_r = tan_reduced_s_lat_r * cos_reduced_s_lat_r;

            let tan_reduced_e_lat_r = (1.-ELLIPSE_WGS84_F) * e_lat_r.tan();
            let cos_reduced_e_lat_r = 1. / (1.+tan_reduced_e_lat_r.powi(2)).sqrt();
            let sin_reduced_e_lat_r = tan_reduced_e_lat_r * cos_reduced_e_lat_r;

            let mut _lambda:f64 = diff_lng_r;
            let mut _lambda_dash:f64 = 0.;

            let mut sin_lng_r:f64 = 0.;
            let mut cos_lng_r:f64 = 0.;
            drop(&sin_lng_r);
            drop(&cos_lng_r);

            let antipodal = diff_lng_r > PI/2. || (s_lat_r - e_lat_r).abs() > PI/2.;

            let mut ang_dist:f64 = if antipodal { PI } else {0.};
            let mut sin_ang_dist:f64 = 0.;
            let mut cos_ang_dist:f64 = if antipodal { -1. } else { 1. };

            let mut sin_azimuth_of_geodesic_at_equator:f64 = 0.;
            let mut cos_2_ang_dist_from_equator_bisect:f64 = 0.;
            let mut cos_sq_azimuth_of_geodesic_at_equator:f64 = 0.;

            // Dropping a useless reference.
            // Just to get around the compiler "value never used" check.
            drop(&sin_azimuth_of_geodesic_at_equator);

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

                if sin_sq_ang_dist.abs() < 1e-24 { break }

                sin_ang_dist = sin_sq_ang_dist.sqrt();
                cos_ang_dist = {
                    sin_reduced_s_lat_r*sin_reduced_e_lat_r
                    + cos_reduced_s_lat_r*cos_reduced_e_lat_r*cos_lng_r
                };

                ang_dist = sin_ang_dist.atan2(cos_ang_dist);

                sin_azimuth_of_geodesic_at_equator = {
                    cos_reduced_s_lat_r*cos_reduced_e_lat_r*sin_lng_r/sin_ang_dist
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

                if (_lambda-_lambda_dash).abs() <= 1e-12 { break }
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

            return Some(ELLIPSE_WGS84_B*_a*(ang_dist-delta_ang_dist))
        } else {
            return None
        }
    }
}
impl CheckDistance for Vincenty {
    fn within_distance(
        s:&LatLng,
        e:&LatLng,
        distance:f64,
    )->bool {
        return match Self::distance(s, e){
            Some(measured) => measured <= distance,
            None => false,
        }
    }
}
