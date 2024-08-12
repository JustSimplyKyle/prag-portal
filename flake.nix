{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devenv.url = "github:cachix/devenv";
    # Enable Rust in Devenv
    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      devenv,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ devenv.flakeModule ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { pkgs, ... }:
        {
          devenv.shells.default = {
            languages.rust.enable = true;
            languages.rust.channel = "stable";
            packages = with pkgs; [
              dioxus-cli
              pkg-config

              # Extra Libraries required by Tauri/Prag-Portal
              # Formatted with "package # 'system library'/'rust crate'"
              # To pass `cargo check`
              openssl # openssl/openssl-sys

              atk # atk/atk-sys
              cairo # cairo/cairo-sys-rs
              glib # glib-2.0/glib-sys
              gdk-pixbuf # gdk-pixbuf-2.0/gdk-pixbuf-sys
              gtk3 # gdk-3.0/gdk-sys
              pango # pango/pango-sys
              webkitgtk_4_1 # javascriptcore-4.1/javascriptcore-rs-sys
              libsoup_3 # libsoup-3.0/soup3-sys

              # To pass `cargo run`
              xdotool # xdo
            ];
          };
        };
    };
}
