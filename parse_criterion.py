#!/usr/bin/env python

import json
import sys
import numpy as np
from scipy.stats import t

if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <json_file>")
    sys.exit(1)

json_file = sys.argv[1]

with open(json_file, 'r') as f:
    data = json.load(f)

times = [time / iters for time, iters in zip(data['times'], data['iters'])]

# Calculate sample mean and sample standard deviation
sample_mean = np.mean(times)
sample_std = np.std(times, ddof=1)  # Use Bessel's correction for sample std

# Calculate percentiles
percentile_lower = np.percentile(times, 2.5)
percentile_upper = np.percentile(times, 97.5)

# Calculate distances from lower and upper bounds to the mean
distance_lower_to_mean = sample_mean - percentile_lower
distance_upper_to_mean = percentile_upper - sample_mean

def convert_time(time_ns):
    units = [('ms', 1000000), ('us', 1000), ('ns', 1)]
    for unit, divisor in units:
        if time_ns >= divisor:
            return f"{time_ns / divisor:.2f}{unit}"

# Output the results
print("Sample Mean Time:", convert_time(sample_mean))
print("Sample Standard Deviation:", convert_time(sample_std))
print("95% Confidence Interval (Percentiles):", [convert_time(percentile_lower), convert_time(percentile_upper)])
print("Distance from Lower Bound to Mean:", convert_time(distance_lower_to_mean))
print("Distance from Upper Bound to Mean:", convert_time(distance_upper_to_mean))