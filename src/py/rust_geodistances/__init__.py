# -*- coding: utf-8 -*-
"""
============================================
 Rust-accelerated Geodistances Calculations
============================================

Python package for Geodistance calculations with both Haversine and Vincenty, using a Rust backend.

This project includes a Rust binary backend:
- :mod:`lib_rust_geodistances` which can be loaded as::

    from rust_geodistances import bin
"""

from . import lib_rust_geodistances as bin

CalculationMethod = bin.CalculationMethod
"""
Psuedo-Enum class for passing as the ``method`` argument to Rust functions.

.. note::
    This class is defined in Rust, which does not have access to Python's actual
    :class:`~enum.Enum` class. While this class behave similarly to a Python
    Enum, it is NOT a subclass of :class:`enum.Enum`.

The main 3 supported members are:
- :attr:`CalculationMethod.HAVERSINE`
- :attr:`CalculationMethod.VINCENTY`
- :attr:`CalculationMethod.CARTESIAN`
"""

distance = bin.distance
"""
Calculating distance beteen two points.

Parameters
----------
source : Tuple[np.float64, np.float64]
    Source Coordinates.

dest : Tuple[np.float64, np.float64]
    Destination Coordinates.

method : CalculationMethod
    A member of :class:`.CalculationMethod` indicating the calculation algorithm to be
    used.

Returns
-------
float
    A floating point number indicating the distance between the two points.
"""

distance_map = bin.distance_map
"""
Map distance between two array of Latitude/Longitudes.

Parameters
----------
input : List[ List[ Tuple[ np.float64, np.float64 ] ] ]
    An :class:`list` or :class:`tuple` of 1 or 2 members, each being a :class:`list` of
    Latitude/Longitudes pair in :class:`np.float64` format.

method : CalculationMethod
    A member of :class:`.CalculationMethod` indicating the calculation algorithm to be
    used.

Returns
-------
List[ List[ np.float64 ] ]
    The distance map in kilometres.
"""

offset = bin.offset
"""
New coordinates by offsetting from another via distance and bearing.

Parameters
----------
start : Tuple[np.float64, np.float64]
    Starting Coordinates.

distance : np.float64
    Distance to the new point.

bearing : np.float64
    Bearing to the new point; 0º being North.

method : CalculationMethod
    A member of :class:`.CalculationMethod` indicating the calculation algorithm to be
    used.

Returns
-------
Tuple[float, float]
    A floating point number indicating the distance between the two points.
"""
