use crate::data::structs::LatLng;
use crate::geodistances::traits::{CalculateDistance, CheckDistance};

use std::f64::consts::PI;


const RADIUS:f64 = 6373.0;

/// Haversine calculation
/// Assumes spherical world - fast but has errors up to ~0.35%
/// Adapted from https://towardsdatascience.com/better-parallelization-with-numba-3a41ca69452e
pub struct Haversine;
impl CalculateDistance for Haversine {
    fn distance(
        s:&LatLng,
        e:&LatLng,
    )->Option<f64> {

        // Discard calculation if latitude doesn't make any sense
        if -90. <= s.lat && s.lat <= 90. && -90. <= e.lat && e.lat <= 90. {
            let s_lat_r = s.lat * PI / 180.;
            let s_lng_r = s.lng * PI / 180.;
            let e_lat_r = e.lat * PI / 180.;
            let e_lng_r = e.lng * PI / 180.;

            let d = {
                ((e_lat_r - s_lat_r)/2.).sin().powi(2)
                + s_lat_r.cos()*e_lat_r.cos()
                * ((e_lng_r - s_lng_r)/2.).sin().powi(2)
            };

            return Some(2. * RADIUS * d.sqrt().asin())
        } else {
            return None
        }
    }
}
impl CheckDistance for Haversine {
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
