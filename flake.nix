{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        inherit (pkgs) lib;
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            allowUnfree = true;
          };
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [
            "x86_64-unknown-linux-gnu"
          ];
        };
        
        # Define the libraries needed for GUI applications
        libraries = with pkgs; [
          # GUI libs
          libxkbcommon
          libGL
          fontconfig
          wayland
          # x11 libraries
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
          xorg.libxcb
          # Additional libraries that might be needed
          xorg.libXfixes
          xorg.libXrender
          xorg.libXext
          libdrm
          mesa
          # Font and FreeType support
          freetype
          expat
        ];
        
        # Font packages
        fonts = with pkgs; [
          dejavu_fonts
          liberation_ttf
          noto-fonts
          noto-fonts-cjk-sans
          noto-fonts-emoji
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rust
            cmake
            # misc. libraries
            openssl
            pkg-config
            llvmPackages.bintools
            vulkan-loader
          ] ++ libraries ++ fonts;
          
          shellHook = ''
            export LD_LIBRARY_PATH="${lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH"
            export PKG_CONFIG_PATH="${lib.makeSearchPathOutput "dev" "lib/pkgconfig" libraries}:$PKG_CONFIG_PATH"
            
            # Font configuration
            export FONTCONFIG_FILE=${pkgs.fontconfig.out}/etc/fonts/fonts.conf
            export FONTCONFIG_PATH=${pkgs.fontconfig.out}/etc/fonts
            
            # Make fonts available
            export FONTCONFIG_CACHE_DIR="$HOME/.cache/fontconfig"
            mkdir -p $FONTCONFIG_CACHE_DIR
          '';
        };
      }
    );
}
