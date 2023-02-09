with import ./nix/pkgs.nix {};
let project = "metagraph";
in stdenv.mkDerivation rec {
  name = "rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    clang
  ];
}
