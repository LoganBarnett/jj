# Crane-based derivation for the jj binary.
# Called from flake.nix with: import ./cli.nix { inherit craneLib commonArgs; }
{ craneLib, commonArgs }:
craneLib.buildPackage (commonArgs // {
  pname = "jj";
  cargoExtraArgs = "-p jj";
})
