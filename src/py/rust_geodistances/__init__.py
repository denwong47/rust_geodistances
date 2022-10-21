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
