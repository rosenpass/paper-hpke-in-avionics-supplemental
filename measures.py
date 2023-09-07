#!/usr/bin/env python3

import re
import subprocess
import numpy as np
from multiprocessing import Pool
import scipy.stats as st

# Number of times to run the program
num_runs = 30000

def parse_max_resident_size(text):
    pattern = r"Maximum resident set size \(kbytes\): (\d+)"
    match = re.search(pattern, text)
    if match:
        max_resident_size = int(match.group(1))
        return max_resident_size
    else:
        return None

# Function to run the command and capture max resident size
def run_command(_):
    output = subprocess.check_output(["/usr/bin/env", "time", "-v", "./target/x86_64-unknown-linux-musl/release/crypto-test"], stderr=subprocess.STDOUT, text=True)
    return parse_max_resident_size(output)

# Run the loop in parallel
with Pool() as p:
    max_resident_sizes = p.map(run_command, range(num_runs))

# Convert to a NumPy array for easier calculations
max_resident_sizes = np.array(max_resident_sizes)

# Calculate sample mean and sample standard deviation
sample_mean = np.mean(max_resident_sizes)
sample_std = np.std(max_resident_sizes, ddof=1)  # Use Bessel's correction for sample std
sample_se = st.sem(max_resident_sizes)
sample_max = np.max(max_resident_sizes)

# Calculate percentiles
percentile_lower = np.percentile(max_resident_sizes, 2.5)
percentile_upper = np.percentile(max_resident_sizes, 97.5)

# Calculate distances from mean to bounds
distance_lower_to_mean = sample_mean - percentile_lower
distance_mean_to_upper = percentile_upper - sample_mean

print("Sample Mean:", sample_mean, "kB")
print("Sample Standard Deviation:", sample_std, "kB")
print("Sample Maximum:", sample_max, "kB")
print("95% Confidence Interval (Percentiles):", percentile_lower, "kB to", percentile_upper, "kB")
print("Distance from Lower Bound to Mean:", distance_lower_to_mean, "kB")
print("Distance from Mean to Upper Bound:", distance_mean_to_upper, "kB")