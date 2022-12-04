#[allow(unused_imports)]
use duplicate::duplicate_item;

use pyo3::prelude::*;

#[allow(unused_imports)]
use ndarray_numeric::{
    F64Array,
    ArrayWithF64Methods
};

mod calc_models;

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


#[cfg(test)]
mod test_distance {
    use ndarray::{
        arr1,
        arr2,
    };

    use super::calc_models::traits::{
        CalculateDistance,
    };

    use super::calc_models::haversine::Haversine;

    #[test]
    fn test_haversine_distance() {

        let e_latlng = arr2(
            &[
                [-57.97178750223649, 131.42756478116718],
                [67.54068754544909, 52.576877730094196],
                [-69.56671734751839, 90.85189714141865],
                [-70.1710100798327, 176.8970308280446],
                [36.005892942509135, 117.51299907211086],
                [75.43497016541315, -73.9736856298929],
                [-32.1905585966326, -154.16323362161185],
                [-0.132232677257349, 16.894055270567975],
                [-70.38209299634721, 21.672397941287983],
                [17.07784835855101, -166.36356158465745],
                [-33.63030900490873, 51.93746241256801],
                [-17.288438585386118, -42.092958827569504],
                [18.828372806331814, -71.1724127094674],
                [22.75316196889247, 99.7696974044772],
                [-50.53578108044535, 149.73277063509687],
                [1.0334662295934294, -85.50579860283494],
                [62.200593973834316, -90.40172461189],
                [-56.00955619512236, 134.8089041137215],
                [42.044193384597236, 42.24399929247548],
                [-88.17539097858835, 160.69929285477195],
                [-43.1958663447602, 107.77154510968057],
                [-64.3371155373847, -111.04120169056384],
                [18.208215223588297, -138.27848525988813],
                [19.91875811381506, 176.871987907534],
                [-79.38556296230739, 25.90739870386679],
                [-81.53150624387871, -12.187505467979577],
                [-2.784960116604097, -150.9543576851328],
                [42.04469930500329, 23.10184606023256],
                [-18.209870506095555, -163.12081666740934],
                [89.8922878485931, 11.498427194912438],
                [-26.512025591913492, -155.8884144167783],
                [-40.472997088790706, -7.293142306141277]
            ]
        );

        let s_latlng = arr1(&[-57.97178750223649, 131.42756478116718]);

        let distance_haversine = arr1(
            &[
                0.0,
                15355.972751566913,
                2308.405072056323,
                2505.182173041389,
                10530.279544368199,
                17815.985603639052,
                6125.853456177508,
                11409.310093118786,
                4715.271249265813,
                10087.844922193708,
                6296.713095678412,
                11625.232181469633,
                15286.905528776377,
                9443.61238939422,
                1440.9193690362804,
                12904.05385420407,
                17691.747290369894,
                299.1989936012719,
                13810.955353137253,
                3385.6936118761114,
                2319.7124723606366,
                5432.18238458127,
                11732.751975510415,
                9618.70290735097,
                4023.300717013845,
                4351.533043435921,
                9017.48395687606,
                14873.108198861775,
                6859.686447588234,
                16459.6742063208,
                6525.863412671655,
                8416.73664634715
            ]
        );


        // println!(
        //     "{:?}",
        //     Haversine::distance(&s_view, &e_latlng)
        // );

        assert!(
            Haversine::distance(&s_latlng.view(), &e_latlng)
            == distance_haversine
        );
    }
}
