{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
  };
  outputs = { self, nixpkgs, ... } @ inputs: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    packages."${system}".default = pkgs.rustPlatform.buildRustPackage {
      pname = "termarium";
      version = "0.0.1";

      src = ./.;

      cargoBuildOptions = [
        "--release"
      ];

      cargoHash = "sha256-/puQjyxiUQttoQj3aUl8TLsEpIfD9T9+nl9E82dg6lA=";

      installPhase = ''
        mkdir -p $out/bin
        cp target/x86_64-unknown-linux-gnu/release/termarium $out/bin/
      '';

      meta = with pkgs.lib; {
        description = "Termarium";
        license = licenses.mit;
        maintainers = [ maintainers.dan ];
      };
    };

    devShell.${system} = pkgs.mkShell {
      name = "termarium";
      buildInputs = with pkgs; [
        cargo
        rustc
        clippy
      ];
    };
  };
}

