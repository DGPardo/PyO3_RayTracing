import os

# os.environ["MATH_BACKEND"] = "python"
# os.environ["RAYTRACING_BACKEND"] = "python"

# os.environ["MATH_BACKEND"] = "rust"
# os.environ["RAYTRACING_BACKEND"] = "python"

os.environ["MATH_BACKEND"] = "rust"
os.environ["RAYTRACING_BACKEND"] = "rust"

from pathlib import Path
from time import perf_counter_ns
from matplotlib import pyplot as plt
import numpy as np
from scipy import stats
from backend_wrapper import Vector3, Camera, Sphere


CAMERA = Camera(
    light_source=Vector3(-4.0, -4.0, 5.0),
    screen_center=Vector3(0, 0.0, 0.0),
    screen_direction=Vector3(0.0, 0.0, 1.0),
    screen_width_pixels=80,
    screen_height_pixels=60,
    screen_width=16.0,
    screen_height=10.0,
)

SPHERE = Sphere(center=Vector3(0.0, 0.0, 3.0), radius=1.0)


def bench_function(num_evals: int, n_warmup: int, func, *args, **kwargs):
    print("Warming up...")
    for _ in range(n_warmup):
        func(*args, **kwargs)

    eval_times = []
    for evaluation_idx in range(num_evals):
        print(f"Running evaluation {evaluation_idx}/{num_evals}")
        start = perf_counter_ns()
        func(*args, **kwargs)
        end = perf_counter_ns()

        dt_ms = (end - start) / 1_000_000.0
        eval_times.append(dt_ms)
    return eval_times


def plot_bench_results(eval_times, title, save_path: Path):
    """
    Plot the benchmark results as a histogram + KDE

    :param eval_times: list of evaluation times
    :param title: title of the plot

    """
    fig, ax = plt.subplots()
    ax.set_title(title)

    # Create a histogram with the eval times
    n_bins = int(1 + np.ceil(np.log2(len(eval_times))))  # sturge's rule
    ax.hist(
        eval_times, bins=n_bins, density=True, alpha=0.6, color="g", label="Histogram"
    )
    ax.set_xlabel("Time (ms)")

    # Overlay a KDE
    kernel = stats.gaussian_kde(eval_times)
    time_dom = np.linspace(min(eval_times), max(eval_times), 1_000)
    ax.plot(time_dom, kernel(time_dom), "r", label="KDE")
    ax.set_ylabel("Density")

    # Vertical Line with the median
    median_time = np.median(eval_times)
    ax.axvline(
        median_time, color="k", linestyle="--", label=f"Median: {median_time:.2}ms"
    )

    # Create Figure
    ax.legend()
    plt.savefig(save_path)


def bench_raytracing(num_evals: int, n_warmup: int, figure_name: str):
    def benched_function():
        rgbs = CAMERA.fill_screen(SPHERE)

    eval_times = bench_function(num_evals, n_warmup, func=benched_function)
    plot_bench_results(
        eval_times,
        title="Ray Tracing Benchmark",
        save_path=Path(__file__).parent
        / "results_report"
        / "figures"
        / f"{figure_name}.svg",
    )


if __name__ == "__main__":
    bench_raytracing(num_evals=200, n_warmup=20, figure_name="rs_benchmark")
