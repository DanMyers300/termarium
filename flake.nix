{
  description = "Termarium";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: 
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    devShell.${system} = pkgs.mkShell {
      name = "termarium";
      buildInputs = with pkgs; [
        gcc
      ];
    };

    packages.${system}.default = pkgs.callPackage ./nix/default.nix {};
  };
}
