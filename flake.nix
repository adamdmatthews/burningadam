{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        checks = {
          fmt = naersk-lib.buildPackage {
            src = ./.;
            nativeBuildInputs = [ pkgs.rustfmt ];
            mode = "fmt";
          };
          clippy = naersk-lib.buildPackage {
            src = ./.;
            mode = "clippy";
          };
        };
        packages.default = naersk-lib.buildPackage ./.;
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              clippy
              rust-analyzer
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
