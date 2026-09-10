{
  lib,
  cmake,
  stdenv
}:

stdenv.mkDerivation {
  pname = "termarium";
  version = "0.1.0";

  src = lib.sourceByRegex ../. [
    "^src.*"
    "CMakeLists.txt"
  ];

  preInstall = ''
    mkdir -p $out/bin
  '';

  nativeBuildInputs = [ cmake ];
}
