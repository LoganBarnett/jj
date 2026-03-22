#!/usr/bin/env python3
"""Generate Pipeline job config.xml files from Groovy scripts.

Each .groovy file in jenkins/jobs/ becomes a Pipeline job in
runner-homes/jenkins/jobs/<name>/config.xml.  Run this before starting
Jenkins, or after editing a Groovy file, to keep the two in sync.

Jenkins reads config.xml on startup and on a job reload; it does not need
a full restart when only job configuration changes.
"""

import pathlib
import sys

REPO_ROOT = pathlib.Path(__file__).parent.parent
JOBS_SRC = REPO_ROOT / "jenkins" / "jobs"
JOBS_OUT = REPO_ROOT / "runner-homes" / "jenkins" / "jobs"

# Minimal Pipeline job XML.  The Groovy script is wrapped in a CDATA block so
# no XML escaping is required; the only forbidden sequence inside CDATA is
# "]]>" which does not appear in normal pipeline scripts.
TEMPLATE = """\
<?xml version='1.1' encoding='UTF-8'?>
<flow-definition plugin="workflow-job">
  <description>{description}</description>
  <keepDependencies>false</keepDependencies>
  <properties/>
  <definition class="org.jenkinsci.plugins.workflow.cps.CpsFlowDefinition"
              plugin="workflow-cps">
    <script><![CDATA[{script}]]></script>
    <sandbox>true</sandbox>
  </definition>
  <triggers/>
  <disabled>false</disabled>
</flow-definition>
"""

def first_comment(text: str) -> str:
    """Return the first // comment line as a description, or empty string."""
    for line in text.splitlines():
        line = line.strip()
        if line.startswith("//"):
            return line.lstrip("/ ").strip()
        if line:
            break
    return ""

def apply(groovy_path: pathlib.Path) -> None:
    name = groovy_path.stem
    script = groovy_path.read_text()
    description = first_comment(script)
    xml = TEMPLATE.format(description=description, script=script)
    out_dir = JOBS_OUT / name
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "config.xml").write_text(xml)
    print(f"applied: {name}")

def main() -> None:
    groovy_files = sorted(JOBS_SRC.glob("*.groovy"))
    if not groovy_files:
        print("no .groovy files found in jenkins/jobs/", file=sys.stderr)
        sys.exit(1)
    JOBS_OUT.mkdir(parents=True, exist_ok=True)
    for f in groovy_files:
        apply(f)

if __name__ == "__main__":
    main()
