use std::thread;
use std::collections::HashMap;

use pyo3::prelude::*;
// use pyo3::types::PyDict;

use crate::geodistances;

const DEFAULT_WORKERS:usize = 4;

/// Debug information to export the compiled constants to Python.
#[derive(FromPyObject)]
pub struct DebugInformation {
    default_workers: usize,
    workers_count: usize,
    eps: f64,
    radius_spherical: f64,
    wgs84_a: f64,
    wgs84_b: f64,
    wgs84_f: f64,
    vincenty_iterations: u16,
}
impl DebugInformation {
    #[allow(dead_code)]
    pub fn new() -> Self {
        return Self {
            workers_count:      workers_count(),
            default_workers:    DEFAULT_WORKERS,
            eps:                geodistances::config::EPS,
            radius_spherical:   geodistances::config::RADIUS,
            wgs84_a:            geodistances::vincenty::ELLIPSE_WGS84_A,
            wgs84_b:            geodistances::vincenty::ELLIPSE_WGS84_B,
            wgs84_f:            geodistances::vincenty::ELLIPSE_WGS84_F,
            vincenty_iterations:geodistances::vincenty::ITERATIONS,
        };
    }
}
impl ToPyObject for DebugInformation {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let mut pydict = HashMap::new();

        pydict.insert(
            "workers_count".to_string(),
            self.workers_count as f64
        );
        pydict.insert(
            "default_workers".to_string(),
            self.default_workers as f64
        );
        pydict.insert(
            "eps".to_string(),
            self.eps
        );
        pydict.insert(
            "radius_spherical".to_string(),
            self.radius_spherical
        );
        pydict.insert(
            "wgs84_a".to_string(),
            self.wgs84_a
        );
        pydict.insert(
            "wgs84_b".to_string(),
            self.wgs84_b
        );
        pydict.insert(
            "wgs84_f".to_string(),
            self.wgs84_f
        );
        pydict.insert(
            "vincenty_iterations".to_string(),
            self.vincenty_iterations as f64
        );

        return pydict.to_object(py)
    }
}
impl IntoPy<PyObject> for DebugInformation {
    fn into_py(self, py: Python) -> PyObject {
        return self.to_object(py)
    }
}


// Get the number of workers
pub fn workers_count() -> usize {
    return match thread::available_parallelism() {
        Ok(count) => usize::from(count),
        Err(_) => DEFAULT_WORKERS,
    }
}
