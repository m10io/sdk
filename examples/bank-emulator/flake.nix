{
  description = "m10-bank-emulator";
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay?rev=dea24da3d3be23ab53ee80314474afd5fcb03c1c";
    flake-utils.url = "github:numtide/flake-utils";
    cargo2nix.url = "github:cargo2nix/cargo2nix?rev=c149357cc3d17f2849c73eb7a09d07a307cdcfe8";
    nixpkgs.follows = "cargo2nix/nixpkgs";
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
          overlays = [ cargo2nix.overlays.default rust-overlay.overlays.default ];
        };
        m10-protos = import ../../nix/protos.nix { inherit pkgs; };
        rust-overrides = import ../../nix/rust-overrides.nix { inherit m10-protos; };
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustChannel = "1.71.1";
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
