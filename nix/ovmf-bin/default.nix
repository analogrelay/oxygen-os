{ stdenv, fetchurl, arch, ... }: 
let
  inherit (stdenv.hostPlatform) system;

  version = "3057fd0cafd168b2303a739e0c6f8c271024fcaa";
  sources = rec {
    aarch64 = [
      (fetchurl {
        name = "QEMU_EFI";
        url = "https://raw.githubusercontent.com/retrage/edk2-nightly/${version}/bin/RELEASEAARCH64_QEMU_EFI.fd";
        hash = "sha256-A3CslPCx/LSsLF/2rSaiJPZzpmbEPJ1h35qv/K1tKMo=";
      })
      (fetchurl {
        name = "QEMU_VARS";
        url = "https://raw.githubusercontent.com/retrage/edk2-nightly/${version}/bin/RELEASEAARCH64_QEMU_VARS.fd";
        hash = "sha256-i2NMHmvRFgeFC2kRH2xNvRWD270UYNrHLbrL3BpKEwo=";
      })
    ];

    x86_64 = [
      (fetchurl {
        name = "OVMF_CODE";
        url = "https://raw.githubusercontent.com/retrage/edk2-nightly/${version}/bin/RELEASEX64_OVMF_CODE.fd";
        hash = "sha256-ZX203k6//hCnfhkvLCXwgq6zF6drkB2H3XYJYWP12jI=";
      })
      (fetchurl {
        name = "OVMF_VARS";
        url = "https://raw.githubusercontent.com/retrage/edk2-nightly/${version}/bin/RELEASEX64_OVMF_VARS.fd";
        hash = "sha256-XSrDgzcbQIOYrM7n7CfIwJ6lt0oN4M7qZRM4ixW+XR4=";
      })
    ];
  };
  arches = builtins.attrNames sources;
in
stdenv.mkDerivation rec {
  pname = "ovmf-bin";
  inherit version;

  srcs = 
    if (builtins.elem arch arches) then
      sources.${arch}
    else
      throw "Unsupported architecture: ${arch}";

  unpackPhase = builtins.concatStringsSep "\n" ([
    "mkdir -p $out"
  ] ++ builtins.map (x: "cp -v ${x} $out/${x.name}.fd;") srcs);

  passthru = {
    source_files = srcs;
  };
}