/// Calculation Models.
///
/// Each calculation model should be a sub module of its own (e.g. haversine, vincenty)
/// with a distinctly named struct that implements the traits from :mod:`trait`
/// submodule.
///
/// The structs are for core calculations only; any shared code among models not unique
/// to the model should be written in :mod:`compatibility::enums` instead.

pub mod config;
pub mod traits;

pub use config::{
    CalculationSettings,
};

pub mod haversine;
pub mod vincenty;

pub use haversine::{
    Haversine
};
pub use vincenty::{
    Vincenty
};
