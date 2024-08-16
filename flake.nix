{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
  # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        crane = inputs.crane.lib;
        fenix = inputs.fenix.packages;
        toolchain = fenix.${system}.minimal.toolchain;

        # crane: cargo and artifacts manager
        craneLib = crane.${system}.overrideToolchain toolchain;

        buildInputs = with pkgs; [
          pkg-config
        ]
        ++ lib.optionals stdenv.buildPlatform.isDarwin [
          pkgs.libiconv
        ];
      in {
        # `nix develop`
        devShells.default = craneLib.devShell {
          inherit buildInputs;
        };
      }
    );
}
