let
  pkgs = import <nixpkgs> {};
  crossPkgs = pkgs.pkgsCross.x86_64-embedded;

    # Create a wrapper for gdb
  gdb-wrapper = pkgs.writeScriptBin "gdb" ''
    #!${pkgs.stdenv.shell}
    exec ${crossPkgs.buildPackages.gdb}/bin/x86_64-elf-gdb "$@"
  '';
in
  pkgs.mkShell {
    nativeBuildInputs = [
      crossPkgs.buildPackages.bochs
      crossPkgs.buildPackages.gdb
      crossPkgs.buildPackages.binutils
      gdb-wrapper  # Add the wrapper

      pkgs.xorriso
      pkgs.qemu
    ];

    shellHook = ''
    alias gs='git status'
    alias gap='git add -p'
    alias gd='git diff'
    alias gds='git diff --staged'
    '';
  }
