{
  description = "";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }@inputs: let
    overlays = [
      (import rust-overlay)
      (final: prev: {
        jenkins = (prev.jenkins or {}) // {
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
        # Naming this "rust" clobbers some things in nixpkgs or rust-overlay
        # that need the "rust" package to be a specific way.  Just use this.
        rust-local = prev.rust-bin.stable.latest.default.override {
          extensions = [
            # For rust-analyzer and others.  See
            # https://nixos.wiki/wiki/Rust#Shell.nix_example for some details.
            "rust-src"
            "rust-analyzer"
            "rustfmt-preview"
          ];
        };
      })
    ];
    buildInputs = pkgs: (
      [
        pkgs.cargo
        pkgs.jdk
        pkgs.jenkins
        pkgs.rust-local
        pkgs.rustfmt
        pkgs.rustup
        # Required for Jenkins to do its own memory management.  See
        # https://groups.google.com/g/jenkinsci-users/c/rrt25fUJCWY/m/1fY0El6lBwAJ
        # for additional research done on this topic.
        # pkgs.top
      ] ++ pkgs.lib.optionals pkgs.stdenv.targetPlatform.isDarwin [
        pkgs.darwin.apple_sdk.frameworks.Security
        # Needed by something, but it's not apparent what.
        pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
      ]
    );
    shellHook = ''
      export JENKINS_HOME=$PWD/runner-homes/jenkins
      mkdir -p $JENKINS_HOME
    '';
  in {

    devShells.aarch64-darwin.default = let
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit overlays system;
      };
    in pkgs.mkShell {
      buildInputs = (buildInputs pkgs);
      inherit shellHook;
    };

  };
}
