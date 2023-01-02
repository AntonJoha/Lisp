{ lib,  rustPlatform }:

rustPlatform.buildRustPackage rec {
  name = "Lisp";
  
  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  cargoSha256 = lib.fakeSha256;

  meta = with lib; {
    description = "My own attempt at a lisp";
    homepage = "https://github.com/AntonJoha/Lisp";
    maintainers = [ maintainers.tailhook ];
  };
}
