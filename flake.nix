{
  description = "jj - trigger Jenkins builds from the command line";
  inputs = {
    # LLM: Do NOT change this URL unless explicitly directed. This is the
    # correct format for nixpkgs stable (25.11 is correct, not nixos-25.11).
    nixpkgs.url = "github:NixOS/nixpkgs/25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    changelog-roller.url = "github:LoganBarnett/changelog-roller";
    foundation.url = "github:LoganBarnett/rust-template";
    foundation.inputs.nixpkgs.follows = "nixpkgs";
    org-fmt.url = "github:LoganBarnett/org-fmt";
    org-fmt.inputs.nixpkgs.follows = "nixpkgs";
    org-fmt.inputs.rust-overlay.follows = "rust-overlay";
    org-fmt.inputs.crane.follows = "crane";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    crane,
    changelog-roller,
    foundation,
    org-fmt,
  } @ inputs: let
    forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    perSystem = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
        # The darwin cross build links Apple's Security / CoreFoundation
        # frameworks (pulled in by reqwest's rustls-native-certs), so it needs
        # the Apple SDK wired into mkDarwinCrossPackages below.  Evaluating
        # apple-sdk.src is the visible acceptance of Apple's SDK licence, and
        # the SDK is unsupported on the Linux build host, so both flags are set.
        config.allowUnfree = true;
        config.allowUnsupportedSystem = true;
      };
      craneLib =
        (crane.mkLib pkgs).overrideToolchain
        (p: p.rust-bin.stable.latest.default);
      rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          # For rust-analyzer and others.  See
          # https://nixos.wiki/wiki/Rust#Shell.nix_example for some details.
          "rust-src"
          "rust-analyzer"
          "rustfmt"
        ];
      };
      # Workspace binaries this project ships.  jj is CLI-only; the lib crate
      # produces no binary and is not listed here.
      crates = {
        cli = {
          name = "jj";
          binary = "jj";
        };
      };
      commonArgs = {
        src = craneLib.cleanCargoSource self;
        # reqwest uses rustls with the OS cert store (rustls-tls-native-roots),
        # so there is no system OpenSSL to link — which is what lets the musl
        # and cross variants below build without a cross OpenSSL.
        buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin; [
          libiconv
        ]);
        # Run only unit tests (--lib --bins); the integration tests under
        # tests/ need a live Jenkins instance unavailable in the Nix sandbox.
        cargoTestExtraArgs = "--lib --bins";
      };
      rustPackages = foundation.lib.mkRustPackages {
        inherit self pkgs craneLib crates commonArgs;
      };
      # On Linux each binary also gets a statically-linked `<name>-musl`
      # variant; empty on other systems.
      muslPackages = foundation.lib.mkMuslPackages {
        inherit self pkgs system crates crane commonArgs;
      };
      # On Linux each binary also gets a portable dynamic-glibc `<name>-gnu`
      # variant; empty on other systems.
      gnuPortablePackages = foundation.lib.mkGnuPortablePackages {
        inherit self pkgs system crates crane commonArgs;
      };
      # The x86_64-linux build cross-compiles macOS `<key>-<arch>-darwin`
      # variants via zig so a release needs no macOS runner; empty on other
      # systems.
      darwinCrossPackages = foundation.lib.mkDarwinCrossPackages {
        inherit self pkgs system crates crane commonArgs;
        # reqwest's rustls-native-certs links the macOS Security framework, so
        # the zig cross build needs Apple's framework headers and .tbd link
        # stubs from the SDK.
        appleSdk = pkgs.apple-sdk.src;
      };
      # Native Windows PE variants (`<key>-{x86_64,aarch64}-windows`),
      # cross-compiled via llvm-mingw for the gnullvm targets — host-agnostic,
      # so they build on Linux CI runners and on a contributor's Mac alike.
      windowsCrossPackages = foundation.lib.mkWindowsCrossPackages {
        inherit self pkgs system crates crane commonArgs;
      };
      packages =
        rustPackages.packages
        // muslPackages
        // gnuPortablePackages
        // darwinCrossPackages
        // windowsCrossPackages
        // {
          default = craneLib.buildPackage (commonArgs // {pname = "jj";});
        };
      # The arm64 subset of the darwin cross outputs — the only ones re-signed,
      # and so the only ones the signature guard verifies.  Empty except on
      # x86_64-linux.
      aarch64DarwinPackages =
        nixpkgs.lib.filterAttrs
        (name: _: nixpkgs.lib.hasSuffix "-aarch64-darwin" name)
        darwinCrossPackages;
      # The x86_64 subset of the Windows cross outputs, smoke-tested under wine.
      windowsX86Packages =
        nixpkgs.lib.filterAttrs
        (name: _: nixpkgs.lib.hasSuffix "-x86_64-windows" name)
        windowsCrossPackages;

      # jj-specific dev tooling: a local Jenkins for exercising the CLI end to
      # end.  Build a directory of symlinks to plugin .jpi files from the
      # attrset produced by jenkins/plugins.nix.
      jenkinsPluginsDir = let
        plugins =
          import ./jenkins/plugins.nix {inherit (pkgs) fetchurl stdenv;};
      in
        pkgs.linkFarm "jenkins-plugins"
        (pkgs.lib.mapAttrsToList
          (name: drv: {
            name = "${name}.jpi";
            path = drv;
          })
          plugins);
    in {
      inherit packages;
      inherit (rustPackages) apps;
      checks =
        rustPackages.checks
        # The darwin ad-hoc signature guard, present only on x86_64-linux
        # where the zig-cross darwin binaries are produced.
        // nixpkgs.lib.optionalAttrs (aarch64DarwinPackages != {}) {
          darwinSignatures = foundation.lib.mkDarwinSignatureCheck {
            inherit pkgs;
            darwinPackages = aarch64DarwinPackages;
          };
        }
        # Run the x86_64 Windows cross binaries under wine to prove they
        # execute, not merely link.  Gated to x86_64-linux.
        // nixpkgs.lib.optionalAttrs (system == "x86_64-linux") {
          windowsSmoke = foundation.lib.mkWindowsSmokeCheck {
            inherit pkgs;
            windowsPackages = windowsX86Packages;
          };
        };
      devShells = {
        default = pkgs.mkShell {
          buildInputs = [
            # Rust toolchain (compiler, cargo, rustfmt, rust-analyzer).
            rust
            # Prunes stale per-profile artifacts from target/ to reclaim disk.
            pkgs.cargo-sweep
            # Local Jenkins for exercising the CLI (jdk runs it).
            pkgs.jdk
            pkgs.jenkins
            # jq and python3 back the Jenkins helper scripts (.github/scripts,
            # jenkins/).
            pkgs.jq
            pkgs.python3
            # Unified formatter and the per-language binaries it invokes.
            pkgs.treefmt
            pkgs.alejandra
            pkgs.prettier
            pkgs.elmPackages.elm-format
            # Formats org-mode documents (treefmt delegates .org files to it).
            org-fmt.packages.${system}.default
          ];
          shellHook = ''
            export JENKINS_HOME=$PWD/runner-homes/jenkins
            export CASC_JENKINS_CONFIG=$PWD/jenkins/casc
            # Disable the first-run setup wizard; JCasC handles all configuration.
            export JAVA_OPTS="''${JAVA_OPTS:-} -Djenkins.install.runSetupWizard=false"
            mkdir -p "$JENKINS_HOME/plugins"
            # Symlink each plugin from the Nix store into JENKINS_HOME so Jenkins
            # finds them without a manual install step.
            for f in ${jenkinsPluginsDir}/*.jpi; do
              ln -sf "$f" "$JENKINS_HOME/plugins/"
            done
            # Symlink cargo-husky hooks into .git/hooks/ using paths relative
            # to .git/hooks/ so the repo stays valid after moves or copies.
            _git_root=$(git rev-parse --show-toplevel 2>/dev/null)
            if [ -n "$_git_root" ] && [ "$(pwd)" = "$_git_root" ] && [ -d ".cargo-husky/hooks" ]; then
              for _hook in .cargo-husky/hooks/*; do
                [ -x "$_hook" ] || continue
                _name=$(basename "$_hook")
                _dest="$_git_root/.git/hooks/$_name"
                _target=$(${pkgs.coreutils}/bin/realpath --relative-to="$_git_root/.git/hooks" "$(pwd)/$_hook")
                if [ ! -L "$_dest" ] || [ "$(readlink "$_dest")" != "$_target" ]; then
                  ln -sf "$_target" "$_dest"
                  echo "Installed git hook: $_name -> $_target"
                fi
              done
            fi
          '';
          # A runtime marker identifying jj's default dev shell.  A compliance
          # check reads it back with `nix eval`; the `ci` shell carries the
          # same marker with the value "ci".
          RUST_TEMPLATE_SHELL = "default";
        };
        # Minimal shell for the reusable CI workflow (`nix develop .#ci`): the
        # project toolchain plus the release CLIs foundation's mkCiShell
        # supplies (changelog-roller, cargo-semver-checks).
        ci = foundation.lib.mkCiShell {
          inherit pkgs system;
          toolchain = rust;
        };
      };
    });
  in {
    devShells = nixpkgs.lib.mapAttrs (_: p: p.devShells) perSystem;
    packages = nixpkgs.lib.mapAttrs (_: p: p.packages) perSystem;
    apps = nixpkgs.lib.mapAttrs (_: p: p.apps) perSystem;
    checks = nixpkgs.lib.mapAttrs (_: p: p.checks) perSystem;
  };
}
