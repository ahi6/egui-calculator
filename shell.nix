{
  pkgs ? import <nixpkgs> { },
}:
let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
in
pkgs.callPackage (
  {
    stdenv,
    mkShell,
    rustup,
    rustPlatform,
  }:
  mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      rustup
      rustPlatform.bindgenHook
    ] ++ [
      pkgs.python3
      pkgs.libGL
      pkgs.libGLU
#       (pkgs.jetbrains.plugins.addPlugins pkgs.jetbrains.rust-rover ["github-copilot"])
      (pkgs.vscode.fhsWithPackages (ps: with ps; [ rustup zlib openssl.dev pkg-config ]))
    ] ++ [
      pkgs.xorg.libxcb
      pkgs.xorg.libXcursor
      pkgs.xorg.libXrandr
      pkgs.xorg.libXi
      pkgs.pkg-config
    ];
    # libraries here
    buildInputs =
      with pkgs; [
        rustc
        cargo
        rustfmt
        rust-analyzer
        clippy
 #       jetbrains.rust-rover
 	vscode.fhs
        xorg.libX11
        wayland
        libxkbcommon
	trunk
      ];
    RUSTC_VERSION = overrides.toolchain.channel;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    shellHook = ''
      export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
      export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
      export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
      "/run/opengl-driver"
      "/run/opengl-driver-32"
      pkgs.libGL
      pkgs.libGLU
      pkgs.vulkan-loader
      pkgs.egl-wayland
      pkgs.wayland
      pkgs.libxkbcommon
      pkgs.xorg.libXcursor
    ]}:$LD_LIBRARY_PATH"
    '';
  }
) { }
