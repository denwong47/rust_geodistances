#[allow(unused_imports)]
use duplicate::duplicate_item;

use pyo3::prelude::*;

#[allow(unused_imports)]
use ndarray_numeric::{
    F64Array,
    ArrayWithF64Methods
};

mod calc_models;
use calc_models::{
    traits,
};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn func(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_rust_geodistances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(func, m)?)?;
    Ok(())
}


/// ====================================================================================
/// UNIT TESTS
///
/// Test that ndarray_numeric is working.
#[cfg(test)]
#[duplicate_item(
    ArrayType       TestName;
    [ Array2 ]      [test_array2];
    [ ArcArray2 ]   [test_arcarray2];
)]
mod TestName {
    use std::f64::consts;
    use super::*;
    use ndarray::{ArrayType, s};

    #[test]
    fn test_f64array_2d() {
        let arr = F64Array::from_shape_fn(
            (3, 4),
            |x| ((x.0)*4 + (x.1)) as f64 * consts::PI / 180. * 10.
        );

        let ans = ArrayType::from_shape_vec(
            (3, 4),
            vec![
                0.0,                0.17453292519943295, 0.3490658503988659, 0.5235987755982988,
                0.6981317007977318, 0.8726646259971648,  1.0471975511965976, 1.2217304763960306,
                1.3962634015954636, 1.5707963267948966,  1.7453292519943295, 1.9198621771937625
            ]
        ).unwrap();

        assert!(arr==ans);

        let slice = ArrayType::from_shape_vec(
            (2, 2),
            vec![
                0.17364817766693033, 0.3420201433256687,
                0.766044443118978, 0.8660254037844386
            ]
        ).unwrap();

        assert!(&arr.slice(s![0..2, 1..3]).sin() == slice);
    }
}
