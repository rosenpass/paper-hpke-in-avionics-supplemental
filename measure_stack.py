#!/usr/bin/env python3

import argparse
import subprocess
from tqdm import tqdm
from math import log2

def run_program(program_path, size, find_all_segments=False):
    softlimit_option = "-s" if not find_all_segments else "-a"
    try:
        subprocess.run(["softlimit", softlimit_option, str(size), program_path], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        return True
    except subprocess.CalledProcessError as e:
        return False

def find_max_size(program_path, successful_runs, start_size, find_all_segments=False):
    lower_bound = 0
    upper_bound = start_size

    with tqdm(total=log2(lower_bound + upper_bound), desc="Binary Search", leave=False) as search:
        while lower_bound < upper_bound:
            size = (lower_bound + upper_bound) // 2

            for _ in tqdm(range(successful_runs), desc="Run Program", leave=False):
                if not run_program(program_path, size, find_all_segments):
                    lower_bound = size + 1
                    break
            else:
                # All runs were successful, decrease upper bound
                upper_bound = size
            search.update(1)

    return float(upper_bound) // 1024.0

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Find the maximum stack/all_segments size for a given program.")
    parser.add_argument("--runs", "-r", type=int, default=100, help="Number of successful runs to validate functionality")
    parser.add_argument("--limit", "-l", type=int, default=8192 * 1028, help="Starting size limit in bytes")
    parser.add_argument("--all_segments", "-as", action="store_true", help="Find memory consumption of all segments instead of just the size")
    parser.add_argument("program_path", help="Path to the program to run")

    args = parser.parse_args()

    max_value = find_max_size(args.program_path, args.runs, args.limit, args.all_segments)
    if args.all_segments:
        print(f"The maximum memory consumption of all segments is: {max_value}KB")
    else:
        print(f"The maximum stack size is: {max_value}KB")
