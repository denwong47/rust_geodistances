use std::f64::consts::PI;

use crate::data::structs::LatLng;
use crate::geodistances::traits::{CalculateDistance, CheckDistance, OffsetByVector};
use crate::geodistances::config::RADIUS;


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
            let (s_lat_r, s_lng_r) = s.as_rad();
            let (e_lat_r, e_lng_r) = e.as_rad();

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
impl OffsetByVector for Haversine {
    fn offset(
        s:&LatLng,
        distance:f64,
        bearing:f64,
    )->Option<LatLng> {
        let bearing_r = bearing / 180. * PI;

        let (s_lat_r, s_lng_r) = s.as_rad();
        // lat2: =ASIN(SIN(lat1)*COS(d/R) + COS(lat1)*SIN(d/R)*COS(brng))
        // lon2: =lon1 + ATAN2(SIN(brng)*SIN(d/R)*COS(lat1), COS(d/R)-SIN(lat1)*SIN(lat2))

        let e_lat_r = {
            (
                s_lat_r.sin()*(distance/RADIUS)
                + s_lat_r.cos()*(distance/RADIUS)*bearing_r.cos()
            ).asin()
        };

        let e_lng_r = {
            s_lng_r + (
                bearing_r.sin()*(distance/RADIUS).sin()*s_lat_r.cos()
            ).atan2(
                (distance/RADIUS).cos()-s_lat_r.sin()*e_lat_r.sin()
            )
        };

        return Some(LatLng::new_from_rad(
            e_lat_r,
            e_lng_r
        ))
    }
}
