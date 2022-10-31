# -*- coding: utf-8 -*-
import random
from typing import Optional, Tuple

import numpy as np
import pandas as pd
import pytest
from sklearn.metrics.pairwise import haversine_distances

from rust_geodistances import CalculationMethod, bin, offset

REPEAT_EXECUTION: int = 24

random.seed(2)


def sklearn_distance_map(
    input: Tuple[np.ndarray, Optional[np.ndarray]],
    method: CalculationMethod = CalculationMethod.HAVERSINE,
):
    """
    Convert sklearn into a :func:`distance_map` compatible function.

    :func:`sklearn.metrics.pairwise.haversine_distances` is radian based, and it is called
    differently than :func:`distance_map`.
    """
    assert method is CalculationMethod.HAVERSINE, (
        "Only CalculationMethod.HAVERSINE is allowed for method, but "
        f"{repr(method)} found."
    )

    _radius_spherical = bin.debug_info().get("radius_spherical", 6373.0)

    input = [np.array(_item) * np.pi / 180 for _item in input]

    if len(input) == 1:
        _results = haversine_distances(*input, None)
    else:
        _results = haversine_distances(*input)

    return _results * _radius_spherical


@pytest.mark.parametrize("_execution_number", range(REPEAT_EXECUTION))
def test_within_distance_dual_list(
    _execution_number,
    random_coordinate_dual_list,
):
    """
    Check our output against sklearn.
    """
    _methods = {
        "sklearn": sklearn_distance_map,
        "rust": bin.distance_map,
    }

    _output = {
        _name: pd.DataFrame(_method(random_coordinate_dual_list))
        for _name, _method in _methods.items()
    }

    pd.testing.assert_frame_equal(
        _output["sklearn"],
        _output["rust"],
        check_exact=False,
    )


@pytest.mark.parametrize("_execution_number", range(REPEAT_EXECUTION))
def test_within_distance_single_list(
    _execution_number,
    random_coordinate_dual_list,
):
    """
    Check our output against sklearn.
    """
    random_coordinate_single_list = [
        random_coordinate_dual_list[0],
    ]

    print(random_coordinate_single_list)

    test_within_distance_dual_list(_execution_number, random_coordinate_single_list)
