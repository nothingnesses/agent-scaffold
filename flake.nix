{
  description = "agent-scaffold: scaffold the agent workflow into a project";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
    fenix-monthly = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      perSystem =
        {
          pkgs,
          lib,
          ...
        }:
        let
          rustToolchain =
            with inputs.fenix-monthly.packages.${pkgs.stdenv.hostPlatform.system};
            combine [
              latest.clippy
              latest.rustc
              latest.cargo
              latest.rustfmt
              latest.rust-src
            ];

          treefmtEval = inputs.treefmt-nix.lib.evalModule pkgs {
            projectRootFile = "flake.nix";
            programs = {
              nixfmt.enable = true;
              rustfmt = {
                enable = true;
                package = rustToolchain;
              };
              taplo.enable = true;
              prettier = {
                enable = true;
                includes = [
                  "*.md"
                  "*.yml"
                  "*.yaml"
                  "*.json"
                ];
              };
            };
          };
        in
        {
          formatter = treefmtEval.config.build.wrapper;

          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.bashInteractive

              # Rust
              rustToolchain
              pkgs.rust-analyzer
              pkgs.bacon
              pkgs.cargo-edit

              # Debugging
              pkgs.lldb

              # General tooling
              pkgs.just
              pkgs.git
              pkgs.gh
            ];

            env = {
              RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
        };
    };
}
