{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, mozillapkgs }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";

      mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") {};
      rust-channel = mozilla.rustChannelOf {
        date = "2021-05-20";
        channel = "nightly";
        sha256 = "aamsvtsiO6f+SrThu2yudNAVqUShKUIDnocMsTGUo3A=";
      };
      rust = rust-channel.rust;
      rust-src = rust-channel.rust-src;

      naersk-lib = naersk.lib."${system}".override {
        cargo = rust;
        rustc = rust;
      };

      nativeBuildInputs = with pkgs; [ cmake openssl pkg-config ];
      buildInputs = with pkgs; [
        openssl freetype expat vulkan-loader vulkan-tools
        wayland wayland-protocols libxkbcommon gtk3
      ] ++ (with xorg; [
        libX11 libXcursor libXrandr libXi
      ]);
    in rec {
      packages.fluminurs-desktop = naersk-lib.buildPackage {
        pname = "fluminurs-desktop";
        root = ./.;
        inherit nativeBuildInputs buildInputs;
      };
      defaultPackage = packages.fluminurs-desktop;

      apps.fluminurs-desktop = flake-utils.lib.mkApp {
        drv = packages.fluminurs-desktop;
      };
      defaultApp = apps.fluminurs-desktop;

      devShell = pkgs.mkShell {
        nativeBuildInputs = nativeBuildInputs ++ [
          rust
          pkgs.rust-analyzer
          pkgs.rustfmt
        ];
        inherit buildInputs;
        RUST_SRC_PATH = "${rust-src}/lib/rustlib/src/rust/library";
        RUST_LOG = "info";
        RUST_BACKTRACE = 1;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    });
}
