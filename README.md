# `rust_geodistances` Library

![CI Checks](https://github.com/denwong47/rust_geodistances/actions/workflows/CI.yml/badge.svg?branch=main)

> ## **Documentation**:
>
> **Available at [github pages](https://denwong47.github.io/rust_geodistances/).**

Faster great-circle pair-wise calculations for Latitude-Longitude numpy arrays.

Typically, great-circle distances in python are done via ``scikit-learn`` [haversine_distances](https://scikit-learn.org/stable/modules/generated/sklearn.metrics.pairwise.haversine_distances.html#sklearn.metrics.pairwise.haversine_distances) which is written in [Cython](https://github.com/scikit-learn/scikit-learn/blob/ecb9a70e82d4ee352e2958c555536a395b53d2bd/sklearn/metrics/_dist_metrics.pyx.tp#L2620).

This function while being very efficient

- assumes the globe as a perfect sphere,
- has parameters and returns in radians, and
- does not utilise parallelism.

In practice, most great-circle distances are made on the surface of the earth with
Latitude-Longitudes arrays that contains hundreds of thousands of points, just to
establish adjacency among them.

This library allows quicker parallelised Haversine and Vincenty calculations using
a Rust backend.

For example::

```python
>>> import numpy as np
>>> from rust_geodistances import haversine, vincenty
>>> sn = np.random.random((8000, 2)); sn[:,0] = sn[:,0]*180-90; sn[:,1] = sn[:,1]*360-180
>>> sn
array([[  53.1941384 ,  140.67426345],
        [   4.60083066,   62.12562388],
        [  88.99525052,   98.45081558],
        ...,
        [ -22.91316671,  102.70576891],
        [  48.92518649,   45.16867756],
        [ -26.0821552 , -146.45968174]])
>>> haversine.distance(sn, sn)
array([[    0.        ,  8836.43498769,  4010.49447966, ...,
        9219.94214377,  6175.84784262, 11248.13008269],
        [ 8836.43498769,     0.        ,  9405.97201428, ...,
        5363.41625518,  5183.1320447 , 16148.40028959],
        [ 4010.49447966,  9405.97201428,     0.        , ...,
        12443.95394676,  4501.24761429, 12954.72431644],
        ...,
        [ 9219.94214377,  5363.41625518, 12443.95394676, ...,
            0.        ,  9807.83119801, 10793.61322006],
        [ 6175.84784262,  5183.1320447 ,  4501.24761429, ...,
        9807.83119801,     0.        , 17283.1158441 ],
        [11248.13008269, 16148.40028959, 12954.72431644, ...,
        10793.61322006, 17283.1158441 ,     0.        ]])
>>> vincenty.distance(sn, sn)
array([[    0.        ,  8837.11025873,  4023.24193027, ...,
        9193.32732786,  6195.10039743, 11229.3496438 ],
        [ 8837.11025873,     0.        ,  9402.83686779, ...,
        5359.01964487,  5168.7529272 , 16159.90036067],
        [ 4023.24193027,  9402.83686779,     0.        , ...,
        12424.96985134,  4514.2999062 , 12935.09607141],
        ...,
        [ 9193.32732786,  5359.01964487, 12424.96985134, ...,
            0.        ,  9787.14116681, 10811.013432  ],
        [ 6195.10039743,  5168.7529272 ,  4514.2999062 , ...,
        9787.14116681,     0.        , 17278.15903244],
        [11229.3496438 , 16159.90036067, 12935.09607141, ...,
        10811.013432  , 17278.15903244,     0.        ]])
```

These functions accepts and returns `numpy.ndarray`. In the example
above, we created an array of 8000 Latitude-Longitude pairs, and mapped all
the distances among them. The return values are in kilometres (this can be
changed by setting different radius in
`rust_geodistances.CalculationSettings`). In our case, where the two
inputs are identical objects, the backend will only perform calculations for
``x to y`` but not ``y to x``, and instead mirror the result array along the
diagonal to save calculations.

This enables haversine calculations of this library to be faster than
``scikit-learn`` in equivalent calculations (example in iPython on Apple M1 Pro):

```python
>>> %timeit haversine.distance(sn, sn)
450 ms ± 11.6 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)

>>> %timeit sklearn.metrics.pairwise.haversine_distances(sn/180*np.pi, sn/180*np.pi)*6371
5.54 s ± 16.3 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
```

These calculations scale well with number of physical threads. The higher the core
count, the bigger the difference.


# Folder structure

The source code of this library is laid out as follows:

   - ```docs``` - Sphinx documentation folder.
      - ```docs/build``` - Compiled documentation location. This is used by github action.
      - ```docs/source``` - Source reStructuredText files.
   - ```experiments/``` - Not officially part of the package; scripts that assists development.
   - ```src/```
      - ```src/py``` - Python package find path. Each subdirectory will become a python package.
         - ```src/py/rust_geodistances``` - package directory for ```rust_geodistances```.
      - ```src/rust``` - Rust source code. ```lib.rs``` is situated here.
   - ```tests/``` - Pytest directory.
