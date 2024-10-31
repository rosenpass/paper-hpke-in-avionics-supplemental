# HPKE in Avionics: Supplementary material

This repository contains the code used to generate the benchmarks.

The raw data used in the paper is in [benches.txt](./benches.txt).

## Running the benchmarks

This is the base assumption for the rest of this README.**

In this nix flake, various benchmarks are packaged.

**Please make sure that you have a `flakes` enabled [`nix`](https://nixos.org/) available, and that you are connected to the internet (yes we mean just Nix, not NixOS - you may keep your preferred OS).**

Generally speaking, you need to enter the devshell (which makes sure you have all the toolchain requirements, such as rustc, ..., met).
To enter the devshell, you simply run the following command from within this repo:

```bash
nix develop
```

Once you are in the devshell, you can start benchmarking!
First, get an overview of the available benchmarks:

```bash
nix flake show
```

All the apps which start with `bench-` are benchmarks! You can now simply run them, using (executed from this repo's root dir):

```bash
nix run .#bench-runtime
```

You are of course invited to substitute the `bench-runtime` with any other benchmarks name.
