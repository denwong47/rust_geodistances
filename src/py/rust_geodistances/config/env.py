# -*- coding: utf-8 -*-
"""
=========================
 Environment Definitions
=========================
Environment settings, mostly relating to a temporary non-production state e.g.
``pytest`` or ``sphinx`` build.
"""
import os

try:
    PYTEST_IS_RUNNING = int(os.environ.get("PYTEST_RUNNING", 0))
except ValueError as e:
    PYTEST_IS_RUNNING = 0

try:
    SPHINX_IS_BUILDING = int(os.environ.get("SPHINX_BUILD", 0))
except ValueError as e:
    SPHINX_IS_BUILDING = 0
