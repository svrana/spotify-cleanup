{
  description = "Cleanup pipewire-pulse clients leaked by Spotify";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
      ];
      flake = {
        homeModules = rec {
          spotify-cleanup = import ./module.nix;
          default = spotify-cleanup;
        };
      };
      systems = [ "x86_64-linux" ];
      perSystem = { config, pkgs, ... }: {
        devShells.default = pkgs.mkShell { packages =
          [
            pkgs.rustc
            pkgs.cargo
            pkgs.clippy
            pkgs.bashInteractive
          ];
        };

        packages.spotify-cleanup = pkgs.callPackage ./default.nix { };
        packages.default = config.packages.spotify-cleanup;

        overlayAttrs = {
          inherit (config.packages) spotify-cleanup;
        };
      };
    };
}
