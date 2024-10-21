{
  description = "HPKE crypto agility benchmark suite";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, fenix, treefmt-nix, ... }@inputs:
    flake-utils.lib.eachSystem [ "x86_64-linux" "i686-linux" "aarch64-linux" ]
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };

          # universal formatter
          treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;

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
              latest.rust-analyzer
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
            buildInputs = [
              # when compiling statically, we need static openssl
              pkgs.pkgsStatic.openssl
            ];

            env.CARGO_BUILD_TARGET = rust-target;

            # avoid undefined reference to `__memcpy_chk'
            hardeningDisable = [ "fortify" ];

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
              algorithms = [
                "empty"
                "hdkf256"
                "hdkf512"
                "kyber768"
                "kyber768dilithium"
                "xyber1024dilithium_oqs"
                "xyber768_oqs"
                "xyber768_oqs_ghp"
                "xyber768dilithium_oqs"
                "xyber768dilithium_oqs_ghp"
              ];
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
                      cargo build --release --package memory_bench --no-default-features --features ${op},${alg}

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

          # for `nix fmt`
          formatter = treefmtEval.config.build.wrapper;

          # always check these
          checks = {
            formatting = treefmtEval.config.build.check self;
          };

        });
}

