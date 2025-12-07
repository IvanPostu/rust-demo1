let 
  pkgs = import <nixpkgs> { config = { allowUnfree = false; }; };
  PROJECT_ROOT = builtins.toString ./.;
in
pkgs.mkShell {
  name = "app-shell";

  buildInputs = with pkgs; [
    rustc
    cargo
    gcc
    rustfmt
    clippy
  ];

  LANG = "en_US.UTF-8";
  LC_ALL = "en_US.UTF-8";

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  shellHook = ''
        export PROJECT_ROOT="${PROJECT_ROOT}"
  '';
}
