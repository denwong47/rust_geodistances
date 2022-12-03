# -*- coding: utf-8 -*-
"""
================================
 rust_geodistances
================================

Python Library with a Rust backend to calculate Geodistances using both Haversine and Vincenty methods.

This project includes a Rust binary backend:
- :mod:`lib_rust_geodistances` which can be loaded as
  :attr:`~rust_geodistances.bin`.
"""

from . import decorators
from . import lib_rust_geodistances as bin

print("### Init had run for rust_geodistances ###")
