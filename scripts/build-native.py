#!/usr/bin/env python3
"""Build native artifacts for this host only. Does not publish."""
import json, os, platform, shutil, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
system={"Darwin":"darwin","Linux":"linux"}.get(platform.system())
arch={"arm64":"arm64","aarch64":"arm64","x86_64":"x64"}.get(platform.machine())
if not system or not arch: raise SystemExit("This build script supports macOS/Linux ARM64/x64; no cross-target claims")
if system=="darwin":os.environ["MACOSX_DEPLOYMENT_TARGET"]="11.0"
subprocess.run(["cargo","build","--release","--locked","--workspace"],cwd=ROOT,check=True)
ext="dylib" if system=="darwin" else "so"
lib=ROOT/"target/release"/f"libsevenmlabs_numerology.{ext}"
node=ROOT/"target/release"/f"libsevenmlabs_numerology_node.{ext}"
target=f"{system}-{arch}"
def copy(source,destination):
 destination.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(source,destination)
copy(node,ROOT/"bindings/node/native"/target/"numerology.node")
copy(lib,ROOT/"bindings/python/sevenmlabs_numerology/native"/target/lib.name)
rid=f"{'osx' if system=='darwin' else 'linux'}-{arch}"
copy(lib,ROOT/"bindings/dotnet/SevenMLabs.Numerology/runtimes"/rid/"native"/lib.name)
copy(ROOT/"target/release/sevenmlabs-numerology-cli",ROOT/"artifacts/native"/target/"numerology")
for base in [ROOT/"bindings/node",ROOT/"bindings/python",ROOT/"bindings/python/sevenmlabs_numerology",ROOT/"bindings/dotnet/SevenMLabs.Numerology"]:
 for name in ["LICENSE","THIRD-PARTY-NOTICES.txt"]:copy(ROOT/name,base/name)
print(json.dumps({"target":target,"rid":rid,"engineVersion":"1.0.0"}))
