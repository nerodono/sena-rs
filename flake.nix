{
  description = "sena-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        isDarwin = pkgs.lib.strings.hasSuffix "-darwin" system;
        darwinOptions = pkgs.lib.optionalAttrs isDarwin {
          nativeBuildInputs = with pkgs; [
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];
        };
        options = darwinOptions;

        rust = pkgs.rust-bin.nightly.latest.default;
      in
      {
        devShells.default = pkgs.mkShell (options // {
          shellHook = ''
            export PS1="(sena) $PS1"
          '';
          buildInputs = [
            pkgs.just
            (rust.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
          ];
        });
      }
    );
}
