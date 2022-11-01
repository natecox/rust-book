let pkgs = import <nixpkgs> { };
in pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.rust-analyzer
    pkgs.libclang
    pkgs.libiconv
  ];
}
