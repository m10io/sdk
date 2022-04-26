{
  description = "m10-bank-emulator";
  inputs = {
    rust-overlay.url = "github:sadroeck/rust-overlay?rev=f540e0001088700f72a3b2a178c9d8e70616a4a9";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:sphw/nixpkgs?rev=81cbfc8f2a1e218249b7bff74013b63150171496";
    cargo2nix.url = "github:cargo2nix/cargo2nix?rev=e1591efe36f3549c39d4845341f9a994f31f660b";
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
        rustPkgs = pkgs.rustBuilder.makePackageSet' {
          rustChannel = "1.59.0";
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
