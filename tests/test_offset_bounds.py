# -*- coding: utf-8 -*-
import random

import pandas as pd
import pytest

from rust_geodistances import CalculationMethod, bin, offset

random.seed(2)

REPEAT_EXECUTION: int = 500
MAX_COORDINATE_LIST_SIZE: int = 200
MAX_OFFSET_DISTANCE: float = 2000


def create_random_coordinates():
    return (-85 + 170 * random.random(), -180 + 360 * random.random())


def create_random_coordinate_list():
    return [
        create_random_coordinates()
        for _ in range(random.randint(10, MAX_COORDINATE_LIST_SIZE))
    ]


random_coordinate_list = pytest.fixture(create_random_coordinate_list)


@pytest.fixture
def random_coordinate_single_list(random_coordinate_list):
    return [random_coordinate_list]


@pytest.fixture
def random_coordinate_dual_list():
    return [create_random_coordinate_list() for _ in range(2)]


@pytest.fixture
def random_distance():
    return random.uniform(0, MAX_OFFSET_DISTANCE)


random_coordinates = pytest.fixture(create_random_coordinates)


@pytest.fixture
def random_bearing():
    return random.uniform(0, 360)


def check_within_distance(
    coordinate_lists: list, distance: float, method: CalculationMethod
):
    """
    Check the return of :func:`within_distance_map` against raw output of :func:`distance_map`
    """
    _raw_distances = bin.distance_map(coordinate_lists, method=method)
    _within_distances = bin.within_distance_map(
        coordinate_lists, distance=distance, method=method
    )

    if _raw_distances and _within_distances:
        assert (pd.DataFrame(_raw_distances) <= distance).equals(
            pd.DataFrame(_within_distances)
        )


@pytest.mark.parametrize("_execution_number", range(REPEAT_EXECUTION))
@pytest.mark.parametrize(
    "method",
    (
        # CalculationMethod.CARTESIAN,
        CalculationMethod.HAVERSINE,
        CalculationMethod.VINCENTY,
    ),
)
def test_within_distance_dual_list(
    _execution_number,
    random_coordinate_dual_list,
    random_distance,
    method,
):
    """
    Check if within_distance_map checks the distance correctly using 2 random lists.
    """
    check_within_distance(
        random_coordinate_dual_list,
        distance=random_distance,
        method=method,
    )


@pytest.mark.parametrize("_execution_number", range(REPEAT_EXECUTION))
@pytest.mark.parametrize(
    "method",
    (
        # CalculationMethod.CARTESIAN,
        CalculationMethod.HAVERSINE,
        CalculationMethod.VINCENTY,
    ),
)
def test_offset_against_distance(
    _execution_number,
    random_coordinates,
    random_distance,
    random_bearing,
    method,
):
    """
    From one point offset to another, then check their distances.
    """
    _start = random_coordinates
    _dest = bin.offset(_start, random_distance, random_bearing, method=method)

    _calculated_distance = min(
        bin.distance(_start, _dest, method=method),
        bin.distance(_dest, _start, method=method),
    )

    assert _calculated_distance == pytest.approx(random_distance)


# =====================================================================
# Skipped Tests


@pytest.mark.parametrize("_execution_number", range(REPEAT_EXECUTION))
@pytest.mark.parametrize(
    "method",
    (
        # CalculationMethod.CARTESIAN,
        CalculationMethod.HAVERSINE,
        CalculationMethod.VINCENTY,
    ),
)
@pytest.mark.skip(
    reason="This is to test a concept to make within_distance more efficient, "
    "but doesn't work."
)
def test_diagonal_bounds(
    _execution_number, random_coordinates, random_distance, method
):
    """
    This is an experiment to test if we can use diagonals to test distance bounds.

    As it turns out, because the world is not a Cartesian plane, going diagonally by
    ``sqrt(2*distance)`` does not provide any guarantee of the values of the
    corresponding 2 orthogonal vectors.

    Seems like we just have to go with 1 check for each orthogonal direction.
    """
    _ne_offset = offset(
        random_coordinates, (random_distance**2 * 2) ** 0.5, 45, method=method
    )
    _sw_offset = offset(
        random_coordinates, (random_distance**2 * 2) ** 0.5, 225, method=method
    )

    _n_offset = offset(random_coordinates, random_distance, 0, method=method)
    _e_offset = offset(random_coordinates, random_distance, 90, method=method)
    _s_offset = offset(random_coordinates, random_distance, 180, method=method)
    _w_offset = offset(random_coordinates, random_distance, 270, method=method)

    assert _ne_offset[0] >= _n_offset[0], (
        f"Offsetting {random_distance:8,.2f}km from "
        f"{random_coordinates} using diagonal estimation "
        f"had failed the northern bounds: {_ne_offset[0]} is not >= {_n_offset[0]}"
    )
    assert _ne_offset[1] >= _e_offset[1], (
        f"Offsetting {random_distance:8,.2f}km from "
        f"{random_coordinates} using diagonal estimation "
        f"had failed the eastern bounds: {_ne_offset[1]} is not >= {_e_offset[1]}"
    )
    assert _sw_offset[0] <= _s_offset[0], (
        f"Offsetting {random_distance:8,.2f}km from "
        f"{random_coordinates} using diagonal estimation "
        f"had failed the southern bounds: {_sw_offset[0]} is not <= {_s_offset[0]}"
    )
    assert _sw_offset[1] <= _w_offset[1], (
        f"Offsetting {random_distance:8,.2f}km from "
        f"{random_coordinates} using diagonal estimation "
        f"had failed the western bounds: {_sw_offset[1]} is not <= {_w_offset[1]}"
    )
