{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }: {

  } // flake-utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages.${system};
    ovmf = {
      aarch64 = pkgs.callPackage ./nix/ovmf-bin { arch = "aarch64"; };
      x86_64 = pkgs.callPackage ./nix/ovmf-bin { arch = "x86_64"; };
    };
    mkShell = (arch: pkgs.mkShell {
      name = "oxygen-shell";
      buildInputs = with pkgs; [
        clang
        llvmPackages.bintools
        rustup
      ];

      packages = with pkgs; [ 
        qemu
        ovmf.${arch}
      ];

      LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
      shellHook = ''
        export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
        export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
        export OVMF_PATH="${ovmf.${arch}}"
        export QEMU="${pkgs.qemu}/bin/qemu-system-${arch}"
        export TARGET_ARCH="${arch}"
      '';
    });
  in rec {
    packages = {
      ovmf-bin-x86_64 = ovmf.x86_64;
      ovmf-bin-aarch64 = ovmf.aarch64;
    };
    devShells.aarch64 = mkShell "aarch64";
    devShells.x86_64 = mkShell "x86_64";
    devShells.default = devShells.x86_64;
  });
}
