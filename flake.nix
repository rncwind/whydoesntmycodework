{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };
  };

  outputs = { self, flake-utils, naersk, rust-overlay, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        src = ./.;

        naersk' = pkgs.callPackage naersk { };
        version = "0.1";

      in
      rec {
        packages = rec {
          # For `nix build` & `nix run`:
          bin = naersk'.buildPackage {
            pname = "whydoesntmycodework-bin";
            root = src;
            buildInputs = with pkgs; [
              pkg-config
              openssl
            ];
          };

          static = pkgs.stdenv.mkDerivation {
            pname = "whydoesntmycodework-static";
            inherit (bin) version;
            inherit src;

            phases = "installPhase";

            installPhase = ''
              mkdir $out
              cp -r $src/static $out
            '';
          };

          posts = pkgs.stdenv.mkDerivation {
            pname = "whydoesntmycodework-posts";
            inherit (bin) version;
            inherit src;

            phases = "installPhase";

            installPhase = ''
              mkdir $out
              cp -r $src/posts $out
            '';
          };

          default = pkgs.symlinkJoin {
            name = "whydoesntmycodework-${bin.version}";
            paths = [ static bin posts ];
          };
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # Duplicated for rust-analyzer
            pkg-config
            openssl

            # Profiling
            linuxPackages_latest.perf

            # Stuff we actually want.
            rust-bin.stable.latest.default
            cargo-flamegraph
            rust-analyzer
            dhall
            dhall-json
          ];
        };
      }
    );
}
