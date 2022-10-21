# -*- coding: utf-8 -*-
"""
================================
 rust_geodistances
================================

Python package for Geodistance calculations with both Haversine and Vincenty, using a Rust backend.

This project includes a Rust binary backend:
- :mod:`lib_rust_geodistances` which can be loaded as
  :attr:`~rust_geodistances.bin`.
"""

import enum

from . import lib_rust_geodistances as bin


class Algorithm(enum.IntEnum):
    HAVERSINE = 1
    VINCENTY = 2

    CARTESIAN = 101


print("### Init had run ###")
