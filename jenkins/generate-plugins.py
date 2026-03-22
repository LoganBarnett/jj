#!/usr/bin/env python3
"""Generate jenkins/plugins.nix from the Jenkins stable update center.

Usage:
    python3 jenkins/generate-plugins.py > jenkins/plugins.nix

To add a plugin, append its short name to the `wanted` list below and
re-run.  Transitive required dependencies are resolved automatically;
optional ones are skipped.  The script fetches each .hpi file to compute
its SHA-256 hash, so it requires a network connection.
"""

import base64
import hashlib
import sys
import urllib.request
import json

# Top-level plugins.  Transitive required deps are pulled in automatically.
WANTED = [
    "configuration-as-code",
    "workflow-aggregator",
]

UPDATE_CENTER = "https://updates.jenkins.io/stable/update-center.actual.json"

HEADER = """\
# Jenkins plugin definitions — {roots}
# and all of their required transitive dependencies.
#
# To regenerate this file (e.g. after adding a plugin or updating versions):
#
#   python3 jenkins/generate-plugins.py > jenkins/plugins.nix
#
# The generator queries the Jenkins stable update center, resolves transitive
# dependencies, fetches each .hpi, and writes this file with correct hashes.
# Adding a new top-level plugin means editing the `wanted` list at the top of
# generate-plugins.py and re-running it.
{{ fetchurl, stdenv }}:
{{"""

ENTRY = """\
  "{name}" = stdenv.mkDerivation {{
    name = "{name}";
    src = fetchurl {{
      url = "{url}";
      hash = "{hash}";
    }};
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  }};"""


def fetch_update_center(url: str) -> dict:
    print(f"Fetching update center from {url} ...", file=sys.stderr, flush=True)
    with urllib.request.urlopen(url, timeout=30) as r:
        return json.load(r)


def resolve_deps(roots: list[str], plugins: dict) -> list[str]:
    """Return plugins in dependency order with no duplicates."""
    visited: set[str] = set()
    order: list[str] = []

    def visit(name: str) -> None:
        if name in visited:
            return
        visited.add(name)
        p = plugins.get(name)
        if p is None:
            print(f"  warning: plugin '{name}' not found in update center", file=sys.stderr)
            return
        for dep in p.get("dependencies", []):
            if not dep.get("optional", False):
                visit(dep["name"])
        order.append(name)

    for root in roots:
        visit(root)
    return order


def sha256_sri(url: str) -> str:
    req = urllib.request.Request(url, headers={"User-Agent": "generate-plugins.py"})
    with urllib.request.urlopen(req, timeout=60) as r:
        data = r.read()
    digest = hashlib.sha256(data).digest()
    return "sha256-" + base64.b64encode(digest).decode()


def main() -> None:
    data = fetch_update_center(UPDATE_CENTER)
    plugins = data["plugins"]

    all_plugins = resolve_deps(WANTED, plugins)
    total = len(all_plugins)
    print(f"Resolved {total} plugins (including transitive deps).", file=sys.stderr)

    lines = [HEADER.format(roots=", ".join(WANTED))]
    for i, name in enumerate(all_plugins, 1):
        p = plugins[name]
        url = p["url"]
        print(f"  [{i}/{total}] {name} {p['version']} ...", file=sys.stderr, flush=True)
        h = sha256_sri(url)
        lines.append(ENTRY.format(name=name, url=url, hash=h))
    lines.append("}")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
