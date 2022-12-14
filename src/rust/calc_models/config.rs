use std::thread;

/// Radius to use for calculation methods that assumes the world as a sphere.
pub const RADIUS:f64 = 6371.0;

pub const ELLIPSE_WGS84_A:f64 = 6378.137;
pub const ELLIPSE_WGS84_B:f64 = 6356.752314245;
pub const ELLIPSE_WGS84_F:f64 = 1./298.257223563;

pub static EPS:f64    = f64::EPSILON;

pub const DEFAULT_WORKERS:usize = 4;

pub fn workers_count() -> usize {
    return match thread::available_parallelism() {
        Ok(count) => usize::from(count),
        Err(_) => DEFAULT_WORKERS,
    }
}
