/// ==========================================
///  Command Line Arguments and stdin package
/// ==========================================
pub mod conversions;
pub mod enums;

// Import this if you want CalculationMethod to have Python Methods.
pub mod python;

pub use conversions::{
    Array2ToVecVec,
};

pub use enums::{
    CalculationInterfaceInternal,
    CalculationMethod,
};
