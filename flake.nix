{
  description = "";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }@inputs: {

    devShells.aarch64-darwin.default = let
      system = "aarch64-darwin";
      overlays = [
        (import rust-overlay)
        (self: super: {
          jenkins = (super.jenkins or {}) // {
            config = {
              # Options can be found here:
              # https://github.com/NixOS/nixpkgs/blob/master/nixos/modules/services/continuous-integration/jenkins/default.nix
              options = {
                services.jenkins = {
                  enable = true;
                  withCLI = true;
                };
              };
            };
          };
        })
      ];
      pkgs = import nixpkgs {
        inherit overlays system;
      };
      rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          # For rust-analyzer and others.  See
          # https://nixos.wiki/wiki/Rust#Shell.nix_example for some details.
          "rust-src"
          "rust-analyzer"
          "rustfmt-preview"
        ];
      };
    in pkgs.mkShell {
      buildInputs = [
        pkgs.darwin.apple_sdk.frameworks.Security
        pkgs.cargo
        pkgs.jdk
        pkgs.jenkins
        rust
        pkgs.rustfmt
        pkgs.rustup
        # Required for Jenkins to do its own memory management.  See
        # https://groups.google.com/g/jenkinsci-users/c/rrt25fUJCWY/m/1fY0El6lBwAJ
        # for additional research done on this topic.
        # pkgs.top
      ];
      shellHook = ''
        export JENKINS_HOME=$PWD/runner-homes/jenkins
        mkdir -p $JENKINS_HOME
      '';
    };

  };
}
