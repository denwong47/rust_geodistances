# -*- coding: utf-8 -*-
from timeit import timeit

import numpy as np
from sklearn.metrics.pairwise import haversine_distances

from rust_geodistances import bin

ARRAY_SIZE: int = 12000
TEST_REPEATS: int = 1


def gen_degs(size: int) -> np.ndarray:
    _array = np.random.rand(size, 2)

    _array[:, 0] = _array[:, 0] * 170 - 85
    _array[:, 1] = _array[:, 1] * 360 - 180

    return _array


_array_degs = [gen_degs(ARRAY_SIZE) for _ in range(2)]
_array_rads = [_array * np.pi / 180 for _array in _array_degs]

for max_workers in range(2, 16, 2):
    _rust_speed = (
        timeit(
            lambda: bin.distance_map(
                _array_degs, threaded=True, max_workers=max_workers
            ),
            number=TEST_REPEATS,
        )
        / TEST_REPEATS
    )
    print(
        f"Rust:    {_rust_speed:,.6f}s @ on {ARRAY_SIZE:,}x{ARRAY_SIZE:,}, max_workers={max_workers}"
    )

_sklearn_speed = (
    timeit(lambda: haversine_distances(*_array_rads), number=TEST_REPEATS)
    / TEST_REPEATS
)
print(f"Sklearn: {_sklearn_speed:,.6f}s @ on {ARRAY_SIZE:,}x{ARRAY_SIZE:,}")
