use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::default::Default;
use std::hash::{Hash, Hasher};
use std::thread;

use pyo3::prelude::*;

/// Radius to use for calculation methods that assumes the world as a sphere.
pub const RADIUS:f64 = 6371.0;

pub const ELLIPSE_WGS84_A:f64 = 6378.137;
pub const ELLIPSE_WGS84_B:f64 = 6356.752314245;
pub const ELLIPSE_WGS84_F:f64 = 1./298.257223563;

pub const DEFAULT_WORKERS:usize = 4;

pub fn workers_count() -> usize {
    return match thread::available_parallelism() {
        Ok(count) => usize::from(count),
        Err(_) => DEFAULT_WORKERS,
    }
}

#[pyclass(module="rust_geodistances")]
pub struct CalculationSettings{
    pub spherical_radius:f64,
    pub ellipse_a:f64,
    pub ellipse_b:f64,
    pub ellipse_f:f64,
    pub eps:f64,
    pub workers:usize,
}
impl Default for CalculationSettings {
    fn default() -> Self {
        return Self {
            spherical_radius:   RADIUS,
            ellipse_a:          ELLIPSE_WGS84_A,
            ellipse_b:          ELLIPSE_WGS84_B,
            ellipse_f:          ELLIPSE_WGS84_F,
            eps:                f64::EPSILON,
            workers:            workers_count(),
        }
    }
}
#[pymethods]
impl CalculationSettings {
    #[new]
    fn new(
        spherical_radius:Option<f64>,
        ellipse_a:Option<f64>,
        ellipse_b:Option<f64>,
        ellipse_f:Option<f64>,
        eps:Option<f64>,
        workers:Option<usize>,
    ) -> Self {
        let default = Self::default();

        return Self {
            spherical_radius:   spherical_radius.unwrap_or(default.spherical_radius),
            ellipse_a:          ellipse_a.unwrap_or(default.ellipse_a),
            ellipse_b:          ellipse_b.unwrap_or(default.ellipse_b),
            ellipse_f:          ellipse_f.unwrap_or(default.ellipse_f),
            eps:                f64::max(
                                    f64::EPSILON,
                                    eps.unwrap_or(default.eps)
                                ),
            workers:            cmp::max(
                                    1,
                                    workers.unwrap_or(default.workers)
                                ),
        };
    }

    /// Python representation of the settings.
    fn __repr__(&self) -> String {
        let mut params = vec![];

        params.push(format!("{}={:?}", "spherical_radius", self.spherical_radius));
        params.push(format!("{}={:?}", "ellipse_a", self.ellipse_a));
        params.push(format!("{}={:?}", "ellipse_b", self.ellipse_b));
        params.push(format!("{}={:?}", "ellipse_f", self.ellipse_f));
        params.push(format!("{}={:?}", "eps", self.eps));
        params.push(format!("{}={:?}", "workers", self.workers));

        return format!(
            "CalculationSettings({})", params.join(", ")
        );
    }

    /// String representation of the settings.
    fn __str__(&self) -> String {
        return self.__repr__();
    }

    /// Hash the string representation.
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.__repr__().hash(&mut hasher);
        hasher.finish()
    }
}
