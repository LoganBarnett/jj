{
  description = "jj - trigger Jenkins builds from the command line";
  inputs = {
    # LLM: Do NOT change this URL unless explicitly directed. This is the
    # correct format for nixpkgs stable (25.11 is correct, not nixos-25.11).
    nixpkgs.url = "github:NixOS/nixpkgs/25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, rust-overlay, crane }@inputs: let
    forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    overlays = [
      (import rust-overlay)
    ];
    pkgsFor = system: import nixpkgs {
      inherit system;
      overlays = overlays;
    };

    workspaceCrates = {
      cli = {
        name = "jj";
        binary = "jj";
        description = "Jenkins job CLI";
      };
    };

    devPackages = pkgs: let
      rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          # For rust-analyzer and others.  See
          # https://nixos.wiki/wiki/Rust#Shell.nix_example for some details.
          "rust-src"
          "rust-analyzer"
          "rustfmt"
        ];
      };
    in [
      rust
      pkgs.cargo-sweep
      pkgs.jdk
      pkgs.jenkins
      pkgs.jq
      pkgs.openssl
      pkgs.pkg-config
      pkgs.python3
    ];

    # Build a directory of symlinks to plugin .jpi files from the attrset
    # produced by jenkins/plugins.nix.  Mirrors the NixOS jenkins module's
    # plugin installation logic for non-NixOS hosts.
    jenkinsPluginsDir = pkgs:
      let plugins = import ./jenkins/plugins.nix { inherit (pkgs) fetchurl stdenv; };
      in pkgs.linkFarm "jenkins-plugins"
           (pkgs.lib.mapAttrsToList
             (name: drv: { name = "${name}.jpi"; path = drv; })
             plugins);

    shellHook = pkgs: ''
      export JENKINS_HOME=$PWD/runner-homes/jenkins
      export CASC_JENKINS_CONFIG=$PWD/jenkins/casc
      # Disable the first-run setup wizard; JCasC handles all configuration.
      export JAVA_OPTS="''${JAVA_OPTS:-} -Djenkins.install.runSetupWizard=false"
      mkdir -p "$JENKINS_HOME/plugins"
      # Symlink each plugin from the Nix store into JENKINS_HOME so Jenkins
      # finds them without a manual install step.
      for f in ${jenkinsPluginsDir pkgs}/*.jpi; do
        ln -sf "$f" "$JENKINS_HOME/plugins/"
      done
    '';

  in {

    devShells = forAllSystems (system: let
      pkgs = pkgsFor system;
    in {
      default = pkgs.mkShell {
        buildInputs = devPackages pkgs;
        shellHook = shellHook pkgs;
      };
    });

    packages = forAllSystems (system: let
      pkgs = pkgsFor system;
      craneLib = (crane.mkLib pkgs).overrideToolchain
        (p: p.rust-bin.stable.latest.default);

      commonArgs = {
        src = craneLib.cleanCargoSource ./.;
        buildInputs = with pkgs; [
          openssl
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin; [
          libiconv
        ]);
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      };

      cratePackages = pkgs.lib.mapAttrs (key: crate:
        craneLib.buildPackage (commonArgs // {
          pname = crate.name;
          cargoExtraArgs = "-p ${crate.name}";
        })
      ) workspaceCrates;

    in cratePackages // {
      default = craneLib.buildPackage (commonArgs // { pname = "jj"; });
    });

    apps = forAllSystems (system: let
      pkgs = pkgsFor system;
    in pkgs.lib.mapAttrs (key: crate: {
      type = "app";
      program = "${self.packages.${system}.${key}}/bin/${crate.binary}";
    }) workspaceCrates);

  };
}
