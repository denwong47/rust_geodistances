/// ==========================================
///  Command Line Arguments and stdin package
/// ==========================================
pub mod conversions;
pub mod enums;
// pub mod func;

// Import this if you want CalculationMethod to have Python Methods.
pub mod python;

pub use conversions::{
    Array2ToVecVec,
};

pub use enums::{
    CalculationInterfaceInternal,
    CalculationMethod,
};

// pub use func::{
//     distance,
//     distance_from_point,
//     within_distance,
//     within_distance_of_point,
//     indices_within_distance,
//     indices_within_distance_of_point,
//     offset,
//     offset_from_point,
// };

// pub mod stdin;
// pub mod stdout;
// pub mod pickle;
