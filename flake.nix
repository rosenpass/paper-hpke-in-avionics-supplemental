{
  description = "HPKE crypto agility benchmark suite";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { nixpkgs, flake-utils, fenix, ... }@inputs:
    flake-utils.lib.eachSystem [ "x86_64-linux" "i686-linux" "aarch64-linux" ]
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
          my-python-packages = ps:
            with ps; [
              numpy
              pendulum
              scipy
              matplotlib
              tqdm
            ];

          # rust target name of the `system`
          rust-target = pkgs.pkgsStatic.targetPlatform.rust.rustcTarget;

          rust-toolchain = with fenix.packages.${system};
            combine [
              latest.rustc
              latest.cargo
              latest.clippy
              latest.rustfmt
              targets.${rust-target}.latest.rust-std
            ];
        in
        {
          devShells.default = pkgs.mkShell {

            nativeBuildInputs = [
              pkgs.cmake
              pkgs.pkg-config
              pkgs.rustPlatform.bindgenHook
              rust-toolchain

              # development goodies
              pkgs.cargo-watch

              # benchmark goodies
              (pkgs.python3.withPackages my-python-packages)
              pkgs.valgrind
              pkgs.daemontools
            ];
            buildInputs = with pkgs; [
              openssl
              openssl.dev
              pkgsStatic.openssl # when compiling statically for musl, we need static openssl!
            ];
          };

          apps = {
            bench-runtime = flake-utils.lib.mkApp {
              drv = pkgs.writeShellApplication {
                name = "bench-runtime";
                text = ''
                  exec cargo bench "$@"
                '';
              };
            };
          } //
          (
            let
              inherit (nixpkgs.lib.attrsets) cartesianProduct listToAttrs;
              inherit (nixpkgs.lib.strings) concatStringsSep;

              operations = [ "seal" "open" ];
              algorithms = [ "empty" "hdkf" "kyber" "dilithium" ];
            in
            listToAttrs (map
              ({ op, alg }: rec {
                name = concatStringsSep "-" [ "bench" "memory" alg op ];
                value = flake-utils.lib.mkApp {
                  drv = pkgs.writeShellApplication {
                    inherit name;

                    text = ''
                      cd "$(git rev-parse --show-toplevel)"
                      # TODO check that $PWD is  the repo root
                    
                      # make visible, what we are running
                      set -x

                      # build the thing
                      cargo build --target x86_64-unknown-linux-musl --release --package memory_bench --no-default-features --features ${op},${alg}

                      # ./measure_heap.py target/release/memory_bench
                      ./measure_stack.py target/x86_64-unknown-linux-musl/release/memory_bench
                      ./measure_stack.py -as target/x86_64-unknown-linux-musl/release/memory_bench
                    '';
                  };
                };
              })
              (cartesianProduct {
                op = operations;
                alg = algorithms;
              })
            )
          );

          # always check these
          checks = {
            nixpkgs-fmt = pkgs.runCommand "nixpkgs-fmt"
              {
                nativeBuildInputs = [ pkgs.nixpkgs-fmt ];
              } "nixpkgs-fmt --check ${./.}; touch $out";
            cargo-fmt = pkgs.runCommand "cargo-fmt"
              {
                nativeBuildInputs = [ rust-toolchain ];
              } "cd ${./.}; cargo fmt --check; touch $out";
          };

        });
}

