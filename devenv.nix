{ pkgs, lib, inputs, ... }:
let
  nativePackages = with pkgs; [
    openssl zlib fontconfig freetype wayland wayland-scanner wayland-protocols
    libxkbcommon libx11 libxcb libxcursor libxi libxrandr libGL libglvnd
    vulkan-loader mesa dbus alsa-lib libclang
  ];
  runtimeLibraries = with pkgs; [
    libxkbcommon wayland libx11 libxcb libxcursor libxi libxrandr
    vulkan-loader libglvnd libGL fontconfig freetype dbus alsa-lib
  ];
in {
  overlays = [ inputs.rust-overlay.overlays.default ];
  packages = (with pkgs; [
    git just pkg-config cmake ninja gcc python3 ffmpeg cargo-audit cargo-deny
  ]) ++ nativePackages;
  env = {
    PKG_CONFIG_PATH = lib.makeSearchPathOutput "dev" "lib/pkgconfig" nativePackages;
    LD_LIBRARY_PATH = lib.makeLibraryPath runtimeLibraries + ":/run/opengl-driver/lib";
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  };
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-src" ];
  };
  enterTest = ''
    set -euo pipefail
    cargo fmt --check
    just test
  '';
}
