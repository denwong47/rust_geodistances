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

from . import lib_rust_geodistances as bin

print("### Init had run ###")
