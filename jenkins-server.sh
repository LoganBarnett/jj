#!/usr/bin/env sh

if which nix ; then
  listenAddress='localhost'
  listenPort=11990
  prefix='/'
  jenkinsWarPath="$(nix eval \
    -f '<nixpkgs>' \
    --raw "pkgs.jenkins" \
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
