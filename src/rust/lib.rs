#[allow(unused_imports)]
use duplicate::duplicate_item;

use pyo3::prelude::*;

// use ndarray::{
//     arr1,
//     arr2,
// };

#[allow(unused_imports)]
use ndarray_numeric::{
    F64Array,
    F64Array1,
    F64LatLngArray,
    ArrayWithF64Methods,
};

mod compatibility;
// use compatibility::{
//     Array2ToVecVec,
// };

mod calc_models;

// /// Calculates distances from `start` to each of `dest` using the `method` specified.
// ///
// /// The distance calculation itself is not excessively expensive, but the conversion
// /// from Rust Vec<Vec<f64>> into PyList is.
// ///
// /// Try using the other functions such as indices
// #[pyfunction]
// fn distance(
//     start:  Vec<[f64; 2]>,
//     dest:  Vec<[f64; 2]>,
//     method: Option<&compatibility::enums::CalculationMethod>,
//     settings: Option<&calc_models::config::CalculationSettings>,
// ) -> PyResult<Vec<Vec<f64>>> {
//     let s = arr2(&start);
//     let e = arr2(&dest);

//     let results = {
//         compatibility::func::distance(&s, &e, method, settings)
//     };

//     return Ok(
//         // This is not the vanilla ndarray::ArrayBase::Array2ToVecVec,
//         // but the two dimensional implementation in compatibility::conversions.
//         results.to_vec()
//     )
// }

// /// Calculates distances from the `start` point to each of `dest`
// /// using the `method` specified.
// #[pyfunction]
// fn distance_from_point(
//     start: [f64; 2],
//     dest:  Vec<[f64; 2]>,
//     method: Option<&compatibility::enums::CalculationMethod>,
//     settings: Option<&calc_models::config::CalculationSettings>,
// ) -> PyResult<Vec<f64>> {
//     let s = arr1(&start);
//     let e = arr2(&dest);

//     return Ok(
//         compatibility::func::distance_from_point(&s, &e, method, settings)
//                             .to_vec()
//     );
// }

// /// Return array of booleans whether the points, when mapped between the arrays.
// ///
// ///
// #[pyfunction]
// fn within_distance(
//     start:  Vec<[f64; 2]>,
//     dest:  Vec<[f64; 2]>,
//     distance: f64,
//     method: Option<&compatibility::enums::CalculationMethod>,
//     workers: Option<usize>,
//     settings: Option<&calc_models::config::CalculationSettings>,
// ) -> PyResult<Vec<Vec<bool>>> {
//     let s = arr2(&start);
//     let e = arr2(&dest);

//     return Ok(
//         compatibility::func::within_distance(
//                                 &s, &e,
//                                 distance, method,
//                                 settings,
//                             )
//                             .to_vec()
//     );
// }

// #[pyfunction]
// fn within_distance_of_point(
//     start: [f64; 2],
//     dest:  Vec<[f64; 2]>,
//     distance: f64,
//     method: Option<&compatibility::enums::CalculationMethod>,
//     settings: Option<&calc_models::config::CalculationSettings>,
// ) -> PyResult<Vec<bool>> {
//     let s = arr1(&start);
//     let e = arr2(&dest);

//     return Ok(
//         compatibility::func::within_distance_of_point(
//                                 &s, &e,
//                                 distance, method,
//                                 settings,
//                             )
//                             .to_vec()
//     );
// }

// #[pyfunction]
// fn indices_within_distance(
//     start:  Vec<[f64; 2]>,
//     dest:  Vec<[f64; 2]>,
//     distance: f64,
//     method: Option<&compatibility::enums::CalculationMethod>,
//     settings: Option<&calc_models::config::CalculationSettings>,
// ) -> PyResult<Vec<Vec<usize>>> {
//     let s = arr2(&start);
//     let e = arr2(&dest);

//     return Ok(
//         compatibility::func::indices_within_distance(
//                                 &s, &e,
//                                 distance, method,
//                                 settings,
//                             )
//     );
// }

// #[pyfunction]
// fn indices_within_distance_of_point(
//     start: [f64; 2],
//     dest:  Vec<[f64; 2]>,
//     distance: f64,
//     method: Option<&compatibility::enums::CalculationMethod>,
//     settings: Option<&calc_models::config::CalculationSettings>,
// ) -> PyResult<Vec<usize>> {
//     let s = arr1(&start);
//     let e = arr2(&dest);

//     return Ok(
//         compatibility::func::indices_within_distance_of_point(
//                                 &s, &e,
//                                 distance, method,
//                                 settings,
//                             )
//     );
// }

/// A Python module implemented in Rust.
#[pymodule]
fn lib_rust_geodistances(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(distance_from_point, m)?)?;
    // m.add_function(wrap_pyfunction!(distance, m)?)?;

    // m.add_function(wrap_pyfunction!(within_distance_of_point, m)?)?;
    // m.add_function(wrap_pyfunction!(within_distance, m)?)?;

    // m.add_function(wrap_pyfunction!(indices_within_distance_of_point, m)?)?;
    // m.add_function(wrap_pyfunction!(indices_within_distance, m)?)?;

    m.add_class::<compatibility::enums::CalculationMethod>()?;
    m.add_class::<calc_models::config::CalculationSettings>()?;

    Ok(())
}


/// ====================================================================================
/// UNIT TESTS
///
/// Test that ndarray_numeric is working, and out typical latlngs will pick up their
/// traits.
#[cfg(test)]
mod test_f64array_ops {
    use std::f64::consts;
    use super::*;
    use ndarray::{Array2, ArcArray2, s};

    #[duplicate_item(
        ArrayType       TestName;
        [ Array2 ]      [test_array2_f64array_ops];
        [ ArcArray2 ]   [test_arcarray2_f64array_ops];
    )]
    #[test]
    fn TestName() {
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
    use ndarray_numeric::{
        ArrayWithF64Methods,
        F64LatLngArray,
    };

    use crate::calc_models::traits::OffsetByVector;

    use super::calc_models::traits::{
        CalculateDistance,
    };

    use super::calc_models::haversine::Haversine;

    #[allow(non_upper_case_globals)]
    static latlng_array: [[f64; 2]; 32] = [
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
    ];

    #[test]
    fn test_haversine_distance() {
        let e_latlng:F64LatLngArray = arr2(&latlng_array);

        let s_latlng = arr1(&latlng_array[0]);

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

        assert!(
            (
                Haversine::distance_from_point(&s_latlng.view(), &e_latlng, None)
                * 1e10
            ).floor()
            == (&distance_haversine * 1e10).floor()
        );
    }

    #[test]
    fn test_haversine_offset() {

        let s_latlng = arr2(&latlng_array);

        let offset_values:[[f64; 2]; 32] = [
            [39402.93921538783, 63.985495140461836],
            [36792.97174894066, -138.4328427193995],
            [70326.88760700842, -172.90429063935443],
            [5558.087219429853, 160.03305777934145],
            [59822.250414088085, 8.120233744188482],
            [79697.28673609573, -71.26338440756788],
            [3856.3083841981347, 142.9366319480318],
            [77365.45595370354, 62.95485798235316],
            [12270.089047795283, 5.196529722653452],
            [31671.595518064794, 108.67372833256724],
            [15097.966754927442, -166.5079294616418],
            [58520.72430946817, -8.626800463390765],
            [29772.695191929797, -56.53035808941],
            [58621.44777548318, -178.86548769457636],
            [75496.48269741522, 70.48492016315149],
            [17204.21320302763, 121.2488401666343],
            [36126.50762561824, 138.69340885898742],
            [20017.215290128075, 118.91038549497898],
            [39057.82329552727, 99.93492347224321],
            [77383.790883249, 175.7048716193786],
            [29414.3559908986, 171.4417057240948],
            [8897.22841173925, -47.53165812133034],
            [47931.11402925625, 159.4510847142859],
            [21414.093100604045, 173.06247343478458],
            [38269.53860710932, -78.2173494733468],
            [29748.381533705262, -52.37118317174526],
            [77690.15782765877, 114.25046121707123],
            [44958.70384781421, -63.90248986901756],
            [34995.19899242197, -125.32931498130623],
            [61783.91298797287, -158.23259623852027],
            [40797.94829164596, -149.32358432297835],
            [3544.29792821624, -123.57446428341305]
        ];
        let offset = arr2(&offset_values);

        let e_latlng = arr2(
            &[
                [-60.05959946711832, 121.23327802825838],
                [71.16625690976355, 143.0453150342048],
                [17.80755617797547, 98.29971864495008],
                [-58.10780402033902, -32.77401199096721],
                [-34.01993557176394, -62.8287221904518],
                [74.08945564996776, -62.6265159677871],
                [-55.31964643537094, -117.09958074971331],
                [-10.879933375925816, -4.963237615027651],
                [39.86881371688628, 28.02482126986206],
                [21.77913300215357, 113.16748229363384],
                [-9.657663849212724, -118.56205877823646],
                [30.827932574868612, 140.2798491542913],
                [-32.29241000875692, 9.244560115528884],
                [-35.55499472416153, -79.9212598097912],
                [-46.19371014216625, 86.33996725706277],
                [-13.756380152869156, 72.41716978543172],
                [67.69010232492053, 179.50603391380457],
                [56.01880659254954, -45.16111583687075],
                [42.95296701655718, 30.437895699793557],
                [-64.10932719513045, 156.68782905502826],
                [51.52413864096491, 93.99753300437874],
                [7.566929998044202, -158.16779116189014],
                [-47.71905340325736, -108.70930538609741],
                [-7.423821459122962, -1.6075839442218012],
                [-72.91271709222855, 91.27886937795233],
                [-2.7090837012489146, 115.42707876709596],
                [5.9615477279082825, -170.41937796752615],
                [45.021292306233306, -39.48482992576214],
                [9.813194790049806, -127.08339264541718],
                [-74.26384013942868, 169.87281994133855],
                [-32.38944552619114, -160.05373904789394],
                [-50.65663672526631, -51.23972070572535]
            ]
        );

        // println!("Calc: {:?}", (Haversine::offset(&s_latlng, &offset.column(0), &offset.column(1)) * 1e10).floor());
        // println!("Ans:  {:?}", (&e_latlng * 1e10).floor());
        assert!(
            (Haversine::offset(
                &s_latlng,
                &offset.column(0),
                &offset.column(1),
                None,
            ) * 1e10).floor()
            == (&e_latlng * 1e10).floor()
        );
    }
}
