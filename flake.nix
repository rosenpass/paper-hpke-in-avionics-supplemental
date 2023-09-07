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

          my-python-packages = ps: with ps; [ numpy pendulum scipy matplotlib ];

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
              cargo-audit
              cargo-expand
              cargo-tarpaulin
              nixpkgs-fmt
              jq
              (python3.withPackages my-python-packages)
              bc
              parallel
              nodePackages.prettier
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
            ];
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

