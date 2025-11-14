{
  description = "Rust dev environment for NixOS projects";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells = {
        x86_64-linux = {
          default = pkgs.mkShell {
            buildInputs = [
             pkgs.rustup
             pkgs.rustc
             pkgs.cargo
             pkgs.clang
             pkgs.pkg-config
             pkgs.openssl
             pkgs.zlib
          ];

          shellHook = ''
            export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=${pkgs.clang}/bin/clang
            export RUSTFLAGS="-C linker=${pkgs.clang}/bin/clang"
            echo "Rust dev shell ready!"
          '';
        };
      };
    };
  };
}
