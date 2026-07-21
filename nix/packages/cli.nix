# Crane-based derivation for the jj binary.
# Imported by foundation.lib.mkRustPackages as the per-crate override for the
# `cli` key, with { craneLib, commonArgs, pkgs }.  commonArgs already carries
# the shared dependency artifacts and per-crate test scope.
{
  craneLib,
  commonArgs,
  pkgs,
}:
craneLib.buildPackage (commonArgs
  // {
    pname = "jj";
    cargoExtraArgs = "-p jj";
  })
