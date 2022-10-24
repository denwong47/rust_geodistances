# -*- coding: utf-8 -*-
from timeit import timeit

import numpy as np
from sklearn.metrics.pairwise import haversine_distances

from rust_geodistances import bin


def gen_degs(size: int) -> np.ndarray:
    _array = np.random.rand(size, 2)

    _array[:, 0] = _array[:, 0] * 170 - 85
    _array[:, 1] = _array[:, 1] * 360 - 180

    return _array


_array_degs = [gen_degs(200) for _ in range(2)]
_array_rads = [_array * np.pi / 180 for _array in _array_degs]

_rust_speed = timeit(lambda: bin.distance_map(_array_degs), number=3)
print(f"Rust:    {_rust_speed:,.6f}")

_sklearn_speed = timeit(lambda: haversine_distances(*_array_rads), number=3)
print(f"Sklearn: {_sklearn_speed:,.6f}")
