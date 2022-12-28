/// Abstraction/Compatibility module that unifies all the endpoints of `calc_models`.
///
/// While `calc_models` contains implementations of all the bits and pieces required for
/// all core calculations, there needs to be an additional abstraction layer on top to
/// take care of the non-model specific aspects such as:
///
/// - passing the chosen model as a function parameter,
/// - parallelising long serial calculations using chunks,
/// - picking and choosing the correct orientation for parallelisation, or
/// - convenient functions to `collect` iterators, etc.
///
/// .. note::
///     In the future, this module shall perform bounds check for `within_distance` as
///     well.
///
/// The principle is to keep the `calc_models` as close to the calculation model as
/// possible, not having excessive boilerplate codes across multiple models. All the
/// boilerplates shall be unified and carried out in this module.
///

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
    CalculationSettings,    // Re-imported from `calc_models`.
};
