let 
  pkgs = import <nixpkgs> { config = { allowUnfree = false; }; };
  PROJECT_ROOT = builtins.toString ./.;
in
pkgs.mkShell {
  name = "app-shell";

  buildInputs = with pkgs; [
    rustup
    gcc
  ];

  LANG = "en_US.UTF-8";
  LC_ALL = "en_US.UTF-8";

  shellHook = ''
        export PROJECT_ROOT="${PROJECT_ROOT}"
        export RUSTUP_HOME="$PROJECT_ROOT/.rustup"
        export CARGO_HOME="$PROJECT_ROOT/.cargo"
        export PATH="$CARGO_HOME/bin:$PATH"
  '';
}
