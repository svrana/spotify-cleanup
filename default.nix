{ lib
, pipewire
, rustPlatform
}:
let cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in rustPlatform.buildRustPackage {
  inherit (cargoToml.package) version;
  pname = cargoToml.package.name;
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  nativeBuildInputs = [];

  buildInputs = [
    pipewire
  ];
}
