# -*- coding: utf-8 -*-
"""
================================
 rust_geodistances
================================

Python Library with a Rust backend to calculate Geodistances using both Haversine and Vincenty methods.

This project includes a Rust binary backend:

- :mod:`~rust_geodistances.lib_rust_geodistances` which can be loaded as
  :attr:`~rust_geodistances.bin`.
"""

from . import decorators, lib_rust_geodistances
from .lib_rust_geodistances import CalculationMethod, CalculationSettings

bin = lib_rust_geodistances
"""
Alias for :mod:`~rust_geodistances.lib_rust_geodistances`.
"""

haversine = bin.CalculationMethod.HAVERSINE
"""
Enum instance containing Haversine calculations methods.

Use:

- :func:`haversine.distance`
- :func:`haversine.distance_from_point`
"""

vincenty = bin.CalculationMethod.VINCENTY
"""
Enum instance containing Vincenty calculations methods.

Use:

- :func:`vincenty.distance`
- :func:`vincenty.distance_from_point`
"""
