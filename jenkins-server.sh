#!/usr/bin/env sh

# This error can be ignored:
# java.io.IOException: No suitable implementation found: os.name=Mac OS X os.arch=aarch64 sun.arch.data.model=64
# See: https://issues.jenkins.io/browse/JENKINS-50570
# It is speculated that this is because /proc/meminfo doesn't exist (it doesn't
# on macOS) _and_ top doesn't exist (it does, and I'm even using a Linux
# version).  So maybe they both must exit.
if which nix ; then
  listenAddress='localhost'
  listenPort=11990
  prefix='/'
  # Get the architecture+OS "double" (Nix's term) used to identify the current
  # system, and thus make this script platform agnostic.
  double=$(nix eval --impure --raw --expr 'builtins.currentSystem')
  # See https://xeiaso.net/blog/nix-flakes-look-up-package/ for how one gets
  # packages from Nix Flakes (consider "official" documentation).
  jenkinsWarPath="$(nix eval \
    --inputs-from . \
    --raw nixpkgs#legacyPackages.$double.jenkins \
    --extra-experimental-features nix-command \
    --extra-experimental-features flakes
)/webapps/jenkins.war"
  java \
    -jar $jenkinsWarPath \
    --enable-future-java \
    --httpListenAddress=$listenAddress \
    --httpPort=$listenPort \
    --prefix=$prefix \
    -Djava.awt.headless=true
    # Java options for the start of the command.
    # $\\{concatStringsSep " " cfg.extraJavaOptions} \
    # Java optoins after all other Java args are done. Might need a --
    # separator.
    # $\\{concatStringsSep " " cfg.extraOptions}
fi
