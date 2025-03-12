{ pkgs ? import ./nixpkgs.nix }:

let rustSchtuff = with pkgs; [
  cargo
];

toolSchtuff = with pkgs; [
  git
  curl
];

all = rustSchtuff ++ toolSchtuff;

in pkgs.mkShell {
  packages = all;
  inputsFrom = with pkgs; all;
}
