{ stdenv
, rustPlatform
, lib
, pkg-config
, dbus
}:
let
  inherit (lib.sources) sourceByRegex;
  inherit (builtins) fromTOML readFile;
  src = sourceByRegex ./. [ "Cargo.*" "(src)(/.*)?" ];
  cargoToml = (fromTOML (readFile ./Cargo.toml)).package;
in
rustPlatform.buildRustPackage rec {
  pname = cargoToml.name;

  inherit src;
  inherit (cargoToml) version;

  buildInputs = [
    dbus
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  preInstall = ''
    mkdir -p $out/share/dbus-1/system.d
    cp ${./dbus-bluetooth.xml} $out/share/dbus-1/system.d/dbus-bluetooth.conf
  '';

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "btleplug-0.11.6" = "sha256-Y9QZ6er/zaXALiQUUw8mMvzg15Dhz9NsWQ2WAM/ouh0=";
    };
  };
}
