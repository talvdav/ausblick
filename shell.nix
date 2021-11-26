{ pkgs ? import <nixpkgs> { } }:
let
  neededLibs = with pkgs; (with xorg; [ libX11 libXcursor libXrandr libXi ])
                          ++ [ vulkan-loader wayland wayland-protocols libxkbcommon ];
in with pkgs;
  mkShell {
    buildInputs = [
      cargo cmake pkg-config freetype expat libxkbcommon
    ];
    shellHook = ''
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath neededLibs}"
'';
}
