# -*- coding: utf-8 -*-
import functools

import numpy as np
import pytest
import sklearn.metrics.pairwise

from rust_geodistances import CalculationMethod, CalculationSettings

TEST_LENGTH = 5000


@functools.lru_cache
def latlng_array(seed: int = 0) -> np.ndarray:
    rng = np.random.default_rng(seed=seed)

    _array = rng.random((TEST_LENGTH, 2))
    _array[:, 0] = _array[:, 0] * 180 - 90
    _array[:, 1] = _array[:, 1] * 360 - 180

    return _array


@pytest.mark.parametrize(
    ["method"],
    [
        (CalculationMethod.HAVERSINE,),
        # (CalculationMethod.VINCENTY, ),
    ],
)
@pytest.mark.parametrize(
    ["lhs", "rhs"],
    [
        (latlng_array(0), latlng_array(1)),
        (latlng_array(0), latlng_array(0)),
    ],
)
def test_distances(method: CalculationMethod, lhs: np.ndarray, rhs: np.ndarray):
    radius: float = CalculationSettings().spherical_radius

    sk_results = (
        sklearn.metrics.pairwise.haversine_distances(
            lhs / 180 * np.pi, rhs / 180 * np.pi
        )
        * radius
    )
    rs_results = method.distance(lhs, rhs)

    np.testing.assert_almost_equal(sk_results, rs_results)
