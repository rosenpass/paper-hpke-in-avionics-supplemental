{
  description = "ARINC 653 P4 compliant Linux Hypervisor";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "git+https://github.com/numtide/flake-utils.git";
    devshell.url = "github:numtide/devshell";
    fenix = {
      url = "git+https://github.com/nix-community/fenix.git?ref=main";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "git+https://github.com/nix-community/naersk.git";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, utils, fenix, naersk, devshell, ... }@inputs:
    utils.lib.eachSystem [ "x86_64-linux" "i686-linux" "aarch64-linux" ]
      (system:
        let
          lib = nixpkgs.lib;
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ devshell.overlays.default ];
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
          rust-target = pkgs.rust.toRustTarget pkgs.pkgsStatic.targetPlatform;

          rust-toolchain = with fenix.packages.${system};
            combine [
              latest.rustc
              latest.cargo
              latest.clippy
              latest.rustfmt
              targets.${rust-target}.latest.rust-std
            ];
        in
        rec {
          # a devshell with all the necessary bells and whistles
          devShells.default = (pkgs.devshell.mkShell {
            imports = [ "${devshell}/extra/git/hooks.nix" ];
            name = "crypto-test-dev-shell";
            packages = with pkgs; [
              stdenv.cc
              coreutils
              rust-toolchain
              rust-analyzer
              cargo-outdated
              cargo-udeps
              cargo-watch
              cargo-expand
              cargo-tarpaulin
              nixpkgs-fmt
              (python3.withPackages my-python-packages)
              nodePackages.prettier
              valgrind
              daemontools
            ];
            git.hooks = {
              enable = true;
              pre-commit.text = "nix flake check";
            };
            commands = [
              { package = "git-cliff"; }
              { package = "treefmt"; }
              {
                name = "udeps";
                command = ''
                  PATH="${fenix.packages.${system}.latest.rustc}/bin:$PATH"
                  cargo udeps $@
                '';
                help = pkgs.cargo-udeps.meta.description;
              }
              {
                name = "outdated";
                command = "cargo-outdated outdated";
                help = pkgs.cargo-outdated.meta.description;
              }
              {
                name = "audit";
                command = "cargo audit $@";
                help = pkgs.cargo-audit.meta.description;
              }
              {
                name = "expand";
                command = ''
                  PATH="${fenix.packages.${system}.latest.rustc}/bin:$PATH"
                  cargo expand $@
                '';
                help = pkgs.cargo-expand.meta.description;
              }
              {
                name = "runtime_bench";
                command = ''
                  PATH="${fenix.packages.${system}.latest.rustc}/bin:$PATH"
                  cargo bench $@
                '';
                category = "bench";
                help =
                  "Benchmark the runtime performance of all algorithms and operations";
              }
            ] ++ (
              let
                inherit (nixpkgs.lib.attrsets) cartesianProductOfSets;
                operations = [ "seal" "open" ];
                algorithms = [ "empty" "hdkf" "kyber" "dilithium" ];
              in
              map
                ({ op, alg }: {
                  name = "memory_bench_${alg}_${op}";
                  command = ''
                    cd "$PRJ_ROOT"
                    cargo build --target x86_64-unknown-linux-musl --release --package memory_bench --no-default-features --features ${op},${alg}
                    # ./measure_heap.py target/release/memory_bench
                    ./measure_stack.py target/x86_64-unknown-linux-musl/release/memory_bench
                    ./measure_stack.py -as target/x86_64-unknown-linux-musl/release/memory_bench
                  '';
                  help = "Memory benchmark the ${alg} algorithm with ${op}";
                  category = "bench";
                })
                (cartesianProductOfSets { op = operations; alg = algorithms; })
            );
          });

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

