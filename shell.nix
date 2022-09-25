with import <nixpkgs> {};
let src = fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      rev = "0508a66e28a5792fdfb126bbf4dec1029c2509e0";
      sha256 = "sha256-rO/4oymKXU09wG2bcTt4uthPCp1XsBZjxuCJo3yVXNs=";
   };
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
mkShell rec {
  name = "env";
  nativeBuildInputs = [
    clang_14
    gcc11
    pkgconfig
    llvmPackages_latest.bintools
  ];

  buildInputs = [
    gdb
    latest.rustChannels.nightly.rust
    openssl
    rustfmt
    cargo
    udev
    alsaLib
    vulkan-loader
    xlibsWrapper
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
    libxkbcommon
    wayland
    libGL
  ];

  shellHook = ''
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${lib.strings.makeLibraryPath buildInputs}
  '';
}
