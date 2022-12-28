/// Move this to compatibility?

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

pub const MAX_ITERATIONS:usize = 1000;
pub const TOLERANCE:f64 = 1e-12;

pub const DEFAULT_WORKERS:usize = 4;

pub const LONG_1D_ARRAY:usize = 8192;

pub fn workers_count() -> usize {
    return match thread::available_parallelism() {
        Ok(count) => usize::from(count),
        Err(_) => DEFAULT_WORKERS,
    }
}

#[pyclass(module="rust_geodistances")]
/// Data class for use as ``settings`` arguments to calculation methods.
///
/// All parameters are optional; calling this function by keyworded arguments
/// is strongly recommended.
///
/// Parameters
/// ----------
/// spherical_radius: Optional[numpy.float64]
/// ellipse_a: Optional[numpy.float64]
/// ellipse_b: Optional[numpy.float64]
/// ellipse_f: Optional[numpy.float64]
/// tolerance: Optional[numpy.float64]
/// eps: Optional[numpy.float64]
/// workers: Optional[numpy.uint64]
///
/// Returns
/// -------
/// CalculationSettings
///     Instance of :class:`CalculationSettings`, which can be passed as
///     ``settings`` arguments to calculation methods.
pub struct CalculationSettings{
    #[pyo3(get, set)]
    /// Radius of the earth, assuming it is a sphere.
    ///
    /// **Type:** numpy.float64
    ///
    /// Used in Haversine calculations.
    pub spherical_radius:f64,

    #[pyo3(get, set)]
    /// Ellipsoidal Semi-major axis length (``a``).
    ///
    /// **Type:** numpy.float64
    ///
    /// Used in Vincenty calculations.
    pub ellipse_a:f64,

    #[pyo3(get, set)]
    /// Ellipsoidal Semi-minor axis length (``b``).
    ///
    /// **Type:** numpy.float64
    ///
    /// Used in Vincenty calculations.
    pub ellipse_b:f64,

    #[pyo3(get, set)]
    /// Ellipsoidal Inverse Flattening (``1/f``).
    ///
    /// **Type:** numpy.float64
    ///
    /// Used in Vincenty calculations.
    pub ellipse_f:f64,

    #[pyo3(get, set)]
    /// Tolerance threshold for iterative calculations.
    ///
    /// **Type:** numpy.float64
    ///
    /// Positive value only.
    ///
    /// Used in Vincenty calculations. The iterative process of Vincenty will
    /// stop once the difference is below this threshold.
    ///
    /// This check is applied before the ellpisoid datums are applied, thus it
    /// is more related to the radian angular distance instead of linear
    /// distance. This does NOT mean that resultant values will be accurate to
    /// +/- ``tolerance``.
    /// (Roughly it would be +/- ``tolerance`` * ``ellipse_b``)
    ///
    /// Treat this value as arbitrary, only to be compared with itself.
    pub tolerance:f64,

    #[pyo3(get, set)]
    /// Maximum number of iterations regardless of whether tolerance is matched.
    ///
    /// **Type:** numpy.uint64
    ///
    /// Used in Vincenty calculations. The iterative process of Vincenty will
    /// stop once this number of iterations had taken place.
    pub max_iterations:usize,

    #[pyo3(get, set)]
    /// Epsilon value (``ε``).
    ///
    /// **Type:** numpy.float64
    ///
    /// 2 :class:`numpy.float64` values with absolute difference less than the
    /// ``ε`` will be considered identical.
    pub eps:f64,

    #[pyo3(get, set)]
    /// Number of CPU threads to use during parallelised operations.
    ///
    /// **Type:** numpy.u64
    ///
    /// Used in Haversine calculations.
    pub max_serial_1d_array_len:usize,

    #[pyo3(get, set)]
    /// Number of CPU threads to use during parallelised operations.
    ///
    /// **Type:** numpy.u64
    ///
    /// Used in Haversine calculations.
    pub workers:usize,
}
impl Default for CalculationSettings {
    /// Default value.
    ///
    /// Rust only.
    fn default() -> Self {
        return Self {
            spherical_radius:           RADIUS,
            ellipse_a:                  ELLIPSE_WGS84_A,
            ellipse_b:                  ELLIPSE_WGS84_B,
            ellipse_f:                  ELLIPSE_WGS84_F,
            tolerance:                  TOLERANCE,
            max_iterations:             MAX_ITERATIONS,
            eps:                        f64::EPSILON,
            max_serial_1d_array_len:    LONG_1D_ARRAY,
            workers:                    workers_count(),
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
        tolerance:Option<f64>,
        max_iterations:Option<usize>,
        eps:Option<f64>,
        max_serial_1d_array_len:Option<usize>,
        workers:Option<usize>,
    ) -> Self {
        let default = Self::default();

        return Self {
            spherical_radius:   spherical_radius.unwrap_or(default.spherical_radius),
            ellipse_a:          ellipse_a.unwrap_or(default.ellipse_a),
            ellipse_b:          ellipse_b.unwrap_or(default.ellipse_b),
            ellipse_f:          ellipse_f.unwrap_or(default.ellipse_f),
            tolerance:          tolerance.unwrap_or(default.tolerance),
            max_iterations:     max_iterations.unwrap_or(default.max_iterations),
            eps:                f64::max(
                                    f64::EPSILON,
                                    eps.unwrap_or(default.eps)
                                ),
            max_serial_1d_array_len:
                                max_serial_1d_array_len.unwrap_or(default.max_serial_1d_array_len),
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
        params.push(format!("{}={:?}", "tolerance", self.tolerance));
        params.push(format!("{}={:?}", "max_iterations", self.max_iterations));
        params.push(format!("{}={:?}", "eps", self.eps));
        params.push(format!("{}={:?}", "max_serial_1d_array_len", self.max_serial_1d_array_len));
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

    /// Print the current settings to ``stdout``.
    ///
    /// Example
    /// -------
    /// Use :meth:`explain` to print out the values::
    ///
    ///     >>> from rust_geodistances import CalculationSettings
    ///     >>> CalculationSettings(workers=8).explain()
    ///     CalculationSettings:
    ///       - spherical_radius    =                 6371.0
    ///       - ellipse_a           =               6378.137
    ///       - ellipse_b           =         6356.752314245
    ///       - ellipse_f           =  0.0033528106647474805
    ///       - tolerance           =                  1e-24
    ///       - max_iterations      =                   1000
    ///       - eps                 =  2.220446049250313e-16
    ///       - workers             =                      8
    fn explain(&self) {
        let mut params = vec![];

        params.push(format!("  - {:20}= {:>22?}", "spherical_radius", self.spherical_radius));
        params.push(format!("  - {:20}= {:>22?}", "ellipse_a", self.ellipse_a));
        params.push(format!("  - {:20}= {:>22?}", "ellipse_b", self.ellipse_b));
        params.push(format!("  - {:20}= {:>22?}", "ellipse_f", self.ellipse_f));
        params.push(format!("  - {:20}= {:>22?}", "tolerance", self.tolerance));
        params.push(format!("  - {:20}= {:>22?}", "max_iterations", self.max_iterations));
        params.push(format!("  - {:20}= {:>22?}", "eps", self.eps));
        params.push(format!("  - {:20}= {:>22?}", "max_serial_1d_array_len", self.max_serial_1d_array_len));
        params.push(format!("  - {:20}= {:>22?}", "workers", self.workers));

        return println!(
            "CalculationSettings:\n{}", params.join("\n")
        );
    }
}
