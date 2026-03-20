{
  description = "This is the clam shell of clams";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      fenix,
      git-hooks,
    }:
    let
      # Everything that Nix supports right now
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      eachSystem =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
            fenix-pkg = fenix.packages.${system};
          }
        );
    in
    {
      checks = eachSystem (
        {
          pkgs,
          system,
          fenix-pkg,
          ...
        }:
        let
          # Nightly enables a lot of nice things, but mainly it allows us to build with rustfmt
          rust-nightly = fenix-pkg.complete.withComponents [
            "cargo"
            "clippy"
            "rustc"
            "rustfmt"
            "rustc-codegen-cranelift-preview"
          ];
        in
        {
          pre-commit-check = git-hooks.lib.${system}.run {
            src = ./.;
            package = pkgs.prek; # Prek for parellelizism
            default_stages = [ "pre-push" ];
            hooks = {
              # Formatting is PURELY for QOL
              # Consistency is key for building patterns, and to that end, a priority should be enabling reliable dev setups so this doesn't trip up on pre-push
              nixfmt.enable = true;

              rustfmt = {
                enable = true;
                packageOverrides.cargo = rust-nightly;
                packageOverrides.rustfmt = rust-nightly;
              };

              markdownfmt = {
                enable = true;
                name = "hongdown";
                entry = "hongdown --write";
                files = "\\.md$";
                language = "system";
              };

              # The main branch needs to always be green
              # Both passing all tests and avoiding any clippy lints
              testrust = {
                enable = true;
                name = "testrust";
                entry = "cargo nextest run";
                language = "system";
                pass_filenames = false;
                stages = [ "pre-merge-commit" ];
              };

              clippy = {
                enable = true;
                packageOverrides.cargo = rust-nightly;
                packageOverrides.clippy = rust-nightly;
                stages = [
                  "pre-merge-commit"
                  "pre-push"
                ];
              };
            };
          };
        }
      );

      devShells = eachSystem (
        {
          pkgs,
          system,
          fenix-pkg,
        }:
        let
          rust-nightly = fenix-pkg.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rust-docs"
            "rustc"
            "rustfmt"
            "rustc-codegen-cranelift-preview"
          ];
          hooks = self.checks.${system}.pre-commit-check;

          # Sourcing from nushell for our commands
          mkCommand = name: help: category: {
            inherit name help category;
            command = "cd $PRJ_ROOT && nu .config/scripts/${name}.nu \"$@\"";
          };
          libpath = pkgs.lib.makeLibraryPath (
            with pkgs;
            [
              wayland
              glib
              gst_all_1.gstreamer
              gst_all_1.gst-plugins-base
              gst_all_1.gst-plugins-good
              gst_all_1.gst-plugins-bad
              libxkbcommon
            ]
          );
        in
        {
          default = pkgs.mkShell {
            env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${libpath}";
            packages = with pkgs; [
              rust-nightly # Rust nightly toolchain
              pkg-config # Nicer pkg-config
              gcc

              # GSTRREAMER STUFF
              glib
              glib.dev
              gst_all_1.gstreamer
              gst_all_1.gst-plugins-base
              gst_all_1.gst-plugins-good
              gst_all_1.gst-plugins-bad

              cargo-bump # Bump crate versions
              rust-analyzer # Rust LSP server
              flock # For managing shell concurrency
              nixfmt # Nix formatter
              tombi # TOML formatter/linter
              typos # Source code spell checker
              hongdown # Markdown formatting
              marksman # Markdown LSP server
              taplo # TOML LSP/formatter
              cargo-nextest # Next-gen test runner
              nixd # Nix LSP server
              dotacat # Colorful terminal output
              cuelsp
            ];
          };
        }
      );

      # Expose devShell as a package for `nix shell` compatibility
      packages = eachSystem (
        { system, ... }:
        {
          default = self.devShells.${system}.default;
        }
      );

      formatter = eachSystem ({ pkgs, ... }: pkgs.nixfmt-rfc-style);
    };
}
