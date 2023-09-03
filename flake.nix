{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs) lib;
        pkgs = nixpkgs.legacyPackages.${system};
      in
      with pkgs;
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "madn";
          inherit ((lib.importTOML (self + "/Cargo.toml")).package) version;
          src = self;
          cargoLock.lockFile = self + "/Cargo.lock";
        };
        devShells.default = mkShell rec {
          nativeBuildInputs = [ rustc cargo cargo-flamegraph cargo-dist pkg-config cmake fontconfig ];
          buildInputs = [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            mesa
            vulkan-loader
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}

