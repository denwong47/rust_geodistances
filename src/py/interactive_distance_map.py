"""
Crude implementation of an interactive map distance visualisation plot.

"""
from time import perf_counter

import cartopy.crs as ccrs
import matplotlib.pyplot as plt
import numpy as np
from sklearn.metrics.pairwise import haversine_distances
from tqdm import tqdm


if __name__ == "__main__":
    fig, ax = plt.subplots(
        1, 1, figsize=(14, 8), subplot_kw=dict(projection=ccrs.PlateCarree())
    )

    ax.coastlines()
    ax.set_global()

    THRES = 0.5
    """
    Distance threshold.
    """

    N = 100
    xs = np.linspace(-180, 180, N)
    ys = np.linspace(-90, 90, N // 2)

    grid = np.meshgrid(xs, ys)
    grid_arr = np.hstack((grid[1].ravel()[:, None], grid[0].ravel()[:, None]))

    default_mask = np.ones((N, N // 2), dtype=np.bool_)

    last_markers = []
    last_mask = None
    marker_arr = np.empty((N, N // 2), dtype=object)
    for i in range(marker_arr.shape[0]):
        for j in range(marker_arr.shape[1]):
            marker_arr[i, j] = None

    def inverse_haversine(start_pos, vector, target_distance):
        """
        Very very very crude inverse haversine distance function.

        Parameters
        ----------
        start_pos : (float, float)
            Longitude , latitude vector (local) in degrees.
        vector : (float, float)
            Longitude , latitude vector (local) in degrees. Will be normalised.
        target_distance : float
            Target distance.

        Returns
        -------
        (float, float)
            Target longitude, latitude.

        """
        steps = 0

        start_lon, start_lat = start_pos

        start_lat = np.clip(start_lat, -90, 90)
        start_lon = np.clip(start_lon, -180, 180)

        vector = np.array(vector, dtype=np.float64).ravel()
        vector /= np.linalg.norm(vector)

        lon_dir, lat_dir = vector

        distance = 0
        mag = 0

        curr_pos = (start_lat, start_lon)

        while distance < target_distance and steps < 100000 and abs(curr_pos[0]) < 90:
            mag += 5e-3

            curr_pos = (start_lat + lat_dir * mag, start_lon + lon_dir * mag)

            distance = haversine_distances(
                np.deg2rad(np.array([start_lat, start_lon]).reshape(1, 2)),
                np.deg2rad(np.array(curr_pos).reshape(1, 2)),
            )[0][0]

            steps += 1

        # Adjust bounds for plotting purposes (ideally would want [-180. 180], and [-90, 90]).
        print(np.clip(curr_pos[1], -179, 179), np.clip(curr_pos[0], -89, 89))
        return np.clip(curr_pos[1], -179, 179), np.clip(curr_pos[0], -89, 89)

    def draw_points(c="r", mask=default_mask):
        for i, x in enumerate(tqdm(xs, desc="drawing points...")):
            for j, y in enumerate(ys):
                if not mask[i, j]:
                    continue
                if (_marker := marker_arr[i, j]) is not None:
                    # This prevents a build-up of marker instances and thus keeps the
                    # application running smoothly even as the user clicks multiple times.
                    _marker.remove()
                marker_arr[i, j] = ax.plot(x, y, c=c, marker="o", ms=2)[0]

    def onclick(event):
        global last_markers
        global last_mask

        x, y = event.xdata, event.ydata
        # x in [-180, 180]
        # y in [-90, 90]

        if x is not None and y is not None:
            while last_markers:
                last_markers.pop().remove()

            last_markers.extend(ax.plot(x, y, marker="o", ms=12, c="orange"))

            # Bounding box.

            # N / S.

            _x, low_y = inverse_haversine(
                start_pos=(x, y), vector=(0, -1), target_distance=THRES
            )
            assert abs(_x - x) < 1e-8, "sanity check"
            last_markers.extend(ax.plot(x, low_y, marker="o", ms=15, c="C3"))

            _x, upp_y = inverse_haversine(
                start_pos=(x, y), vector=(0, 1), target_distance=THRES
            )
            assert abs(_x - x) < 1e-8, "sanity check"
            last_markers.extend(ax.plot(x, upp_y, marker="o", ms=15, c="C3"))

            # E / W.

            upp_west_x, _ = inverse_haversine(
                start_pos=(x, upp_y), vector=(-1, 0), target_distance=THRES
            )
            last_markers.extend(ax.plot(upp_west_x, upp_y, marker="o", ms=15, c="C3"))

            upp_east_x, _ = inverse_haversine(
                start_pos=(x, upp_y), vector=(1, 0), target_distance=THRES
            )
            last_markers.extend(ax.plot(upp_east_x, upp_y, marker="o", ms=15, c="C3"))

            low_west_x, _ = inverse_haversine(
                start_pos=(x, low_y), vector=(-1, 0), target_distance=THRES
            )
            last_markers.extend(ax.plot(low_west_x, low_y, marker="o", ms=15, c="C3"))

            low_east_x, _ = inverse_haversine(
                start_pos=(x, low_y), vector=(1, 0), target_distance=THRES
            )
            last_markers.extend(ax.plot(low_east_x, low_y, marker="o", ms=15, c="C3"))

            # Global distances.

            print("calculating distances...")
            distances = haversine_distances(
                np.deg2rad(np.array([y, x]).reshape(1, 2)), np.deg2rad(grid_arr)
            )
            curr_mask = (distances < THRES).reshape(N // 2, N).T
            # Clear old points by drawing over them.
            if last_mask is not None:
                draw_points(mask=last_mask, c="b")
            draw_points(mask=curr_mask, c="r")

            last_mask = curr_mask

            fig.canvas.draw_idle()

    draw_points(c="b")
    fig.canvas.mpl_connect("button_press_event", onclick)
    fig.suptitle("Click me!")
    plt.show()
