{
  description = "m10-bank-emulator";
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay?rev=c8e0f1989d31f5d045bc9f7ae4b5733b6c141efe";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?rev=084b922d4c32fe09235def404b7ede58a5ec0458";
    cargo2nix.url = "github:cargo2nix/cargo2nix?rev=c149357cc3d17f2849c73eb7a09d07a307cdcfe8";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, cargo2nix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlay rust-overlay.overlay ];
        };
        m10-protos = import ../../nix/protos.nix { inherit pkgs; };
        rust-overrides = import ../../nix/rust-overrides.nix { inherit m10-protos; };
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustChannel = "1.63.0";
          packageFun = import ./Cargo.nix;
          packageOverrides = rust-overrides;
        };
        m10-bank-emulator = (rustPkgs.workspace.m10-bank-emulator { }).bin;
        integration-tests = (rustPkgs.workspace.integration-tests { compileMode = "test"; }).bin;
        m10-bank-emulator-tests = pkgs.stdenv.mkDerivation rec {
          pname = "m10-bank-emulator-tests";
          version = "1.0.0";
          phases = [ "installPhase" "patchPhase" ];
          installPhase = ''
            mkdir -p $out/bin
            cp ${integration-tests}/bin/integration_tests* $out/bin/tests
          '';
        };
      in
      rec {
        packages = {
          inherit m10-bank-emulator m10-bank-emulator-tests;
          docker.m10-bank-emulator = pkgs.dockerTools.buildLayeredImage {
            name = "m10-bank-emulator";
            contents = [ pkgs.cacert m10-bank-emulator ];
            config = {
              Cmd = [ "/bin/m10-bank-emulator" ];
            };
          };
        };
        defaultPackage = packages.m10-bank-emulator;
      }
    );
}
