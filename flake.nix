{
  description = "rune-material: Material 3 tokens and components for Rune";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane/79fd63528501446749c09f08cad486a58c7d173e";
  };

  outputs = { self, nixpkgs, crane }: let
    lib = nixpkgs.lib;
    systems = [ "x86_64-linux" "aarch64-linux" ];
    forAllSystems = lib.genAttrs systems;
    source = lib.cleanSourceWith {
      src = ./.;
      filter = path: type: let
        name = baseNameOf path;
        cargo = type == "directory" || lib.hasSuffix ".rs" name
          || lib.hasSuffix ".toml" name || name == "Cargo.lock";
        symbol = lib.hasPrefix (toString ./assets/material-symbols) (toString path)
          && (lib.hasSuffix ".svg" name || name == "README.md");
      in !(builtins.elem name [ "vendor" "target" ".git" ".jj" ]) && (cargo || symbol || name == "LICENSE");
    };
    perSystem = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system; };
      craneLib = crane.mkLib pkgs;
      common = {
        src = source;
        strictDeps = true;
        buildInputs = [ pkgs.libxkbcommon ];
        nativeBuildInputs = [ pkgs.pkg-config ];
        cargoVendorDir = craneLib.vendorCargoDeps {
          src = source;
          # Rune vendors crates.io packages containing Cargo.toml.orig.
          # Cargo rejects those reserved backup files when Crane repackages Git crates.
          overrideVendorGitCheckout = packages: drv:
            if lib.any (p: p.name == "rune") packages then
              drv.overrideAttrs (old: {
                preInstall = (old.preInstall or "") + ''
                  find vendor -name Cargo.toml.orig -type f -delete
                '';
              })
            else drv;
        };
      };
      deps = craneLib.buildDepsOnly common;
      runtimeLibraries = with pkgs; [
        libxkbcommon wayland libx11 libxcb libxcursor libxi libxrandr
        vulkan-loader libglvnd fontconfig freetype dbus alsa-lib
      ];
      runtime = {
        nativeBuildInputs = common.nativeBuildInputs ++ [ pkgs.makeWrapper ];
        postInstall = ''
          for program in "$out"/bin/*; do
            if [ -f "$program" ]; then
              wrapProgram "$program" --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath runtimeLibraries}
            fi
          done
          install -Dm644 ${./LICENSE} $out/share/licenses/rune-material/LICENSE
          install -Dm644 ${./assets/material-symbols/LICENSE} $out/share/licenses/material-symbols/LICENSE
          install -Dm644 ${./assets/material-symbols/README.md} $out/share/licenses/material-symbols/README.md
          install -Dm644 ${./DEPENDENCIES.md} $out/share/doc/rune-material/DEPENDENCIES.md
        '';
      };
      library = craneLib.buildPackage (common // runtime // {
        cargoArtifacts = deps;
        pname = "rune-material";
      });
      demo = bin: craneLib.buildPackage (common // runtime // {
        cargoArtifacts = deps;
        pname = "rune-material-${bin}";
        cargoBuildExtraArgs = "--bin ${bin}";
      });
      gallery = demo "gallery";
      dms-theme = demo "dms-theme";
      calculator = demo "calculator";
      tests = craneLib.cargoTest (common // {
        cargoArtifacts = deps;
        pname = "rune-material-tests";
      });
      formatting = craneLib.cargoFmt { src = source; };
      clippy = craneLib.cargoClippy (common // {
        cargoArtifacts = deps;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      });
    in {
      packages = { default = gallery; inherit library gallery dms-theme calculator; };
        checks = { inherit library gallery dms-theme calculator tests formatting clippy; };
      apps = lib.genAttrs [ "gallery" "dms-theme" "calculator" ] (name: {
        type = "app";
        meta.description = "rune-material ${name} demo";
        program = "${{ inherit gallery dms-theme calculator; }.${name}}/bin/${name}";
      });
      devShell = craneLib.devShell {
        checks = { inherit tests; };
        packages = [ pkgs.just pkgs.libxkbcommon pkgs.pkg-config pkgs.cargo-audit pkgs.cargo-deny ];
        LD_LIBRARY_PATH = lib.makeLibraryPath runtimeLibraries;
      };
    });
  in {
    # Consumers such as Codringher build this source, not a copied UI crate.
    lib.source = source;
    packages = forAllSystems (system: perSystem.${system}.packages);
    checks = forAllSystems (system: perSystem.${system}.checks);
    apps = forAllSystems (system: perSystem.${system}.apps // {
      default = perSystem.${system}.apps.gallery;
    });
    devShells = forAllSystems (system: { default = perSystem.${system}.devShell; });
  };
}
