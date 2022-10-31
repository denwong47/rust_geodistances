# -*- coding: utf-8 -*-
import random

import pytest

from rust_geodistances import config

config.env.PYTEST_IS_RUNNING = 1

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
