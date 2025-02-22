{
  description = "Demo for veilid";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = {self, nixpkgs, flake-utils}: flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs {inherit system;};
  in {
    packages = {};
    devShells.default = pkgs.mkShell {
      packages = [
      pkgs.rustc
      pkgs.cargo
      pkgs.rustfmt
      pkgs.clippy
      pkgs.nil
      pkgs.rust-analyzer
      ];
    };
  });
}
