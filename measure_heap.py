#!/usr/bin/env python3

import subprocess
import re
import argparse

# Function to format memory size
def format_memory_size(size_bytes):
    units = ['B', 'KB', 'MB', 'GB', 'TB']
    unit_index = 0
    while size_bytes >= 1024 and unit_index < len(units) - 1:
        size_bytes /= 1024
        unit_index += 1
    return f"{size_bytes:.2f} {units[unit_index]}"

def run_program_with_valgrind(program_path, command_args):
    # Run the program with valgrind, redirect stderr to DEVNULL, and capture stdout
    valgrind_command = ['valgrind', '--tool=massif', '--pages-as-heap=yes', '--massif-out-file=/dev/stdout', program_path] + command_args
    process = subprocess.Popen(valgrind_command, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
    stdout, _ = process.communicate()

    # Convert the captured output to a string
    output_str = stdout.decode('utf-8')

    # Extract the 'mem_heap_B' lines from the output
    mem_heap_lines = [line.strip() for line in output_str.split('\n') if 'mem_heap_B' in line]

    # Extract the values of 'mem_heap_B' using regular expressions
    mem_heap_values = [int(re.search(r'mem_heap_B=(\d+)', line).group(1)) for line in mem_heap_lines]

    # Sort the values in ascending order and get the maximum
    max_mem_heap = max(mem_heap_values)

    max_mem_heap_formatted = format_memory_size(max_mem_heap)

    print(f"The maximum heap is: {max_mem_heap_formatted}")

def main():
    parser = argparse.ArgumentParser(description='Run a program with Valgrind and measure memory usage.')
    parser.add_argument('program_path', help='Path to the program to run with Valgrind')
    parser.add_argument('command_args', nargs='*', help='Additional command-line arguments for the program')

    args = parser.parse_args()
    run_program_with_valgrind(args.program_path, args.command_args)

if __name__ == "__main__":
    main()
