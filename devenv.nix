{ pkgs, ... }:

{
  languages.rust.enable = true;

  packages = [
    pkgs.fontconfig
    pkgs.freetype
  ];
}
