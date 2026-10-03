{
  description = "Unofficial Elgato Command Centre CLI";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: rec {
        open-ecc = pkgs.rustPlatform.buildRustPackage {
          pname = "open-ecc";
          version = (pkgs.lib.importTOML ./Cargo.toml).workspace.package.version;
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [
            "-p"
            "open_ecc_cli"
          ];
          doCheck = false;
          nativeBuildInputs = [ pkgs.perl ];
          meta = {
            description = "Unofficial Elgato Command Centre cross-platform CLI";
            homepage = "https://github.com/mbwilding/open-ecc";
            license = pkgs.lib.licenses.mit;
            mainProgram = "ecc";
          };
        };
        default = open-ecc;
      });

      overlays.default = final: _prev: {
        open-ecc = self.packages.${final.stdenv.hostPlatform.system}.open-ecc;
      };

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer
            perl
          ];
        };
      });
    };
}
