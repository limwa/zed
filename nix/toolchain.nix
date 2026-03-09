{ inputs, ... }:
pkgs:
let
  rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs;
in
pkgs.callPackage ./build.nix {
  crane = inputs.crane.mkLib pkgs;
  rustToolchain = rustBin.fromRustupToolchainFile ../rust-toolchain.toml;
  commitSha = let
      rev = inputs.self.rev or inputs.self.dirtyRev or null;
    in
      if rev == null then
        null
      else 
        builtins.substring 0 40 rev;
}
