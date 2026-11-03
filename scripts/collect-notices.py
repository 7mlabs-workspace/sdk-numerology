#!/usr/bin/env python3
"""Preserve license texts for all locked registry dependencies, including build tools."""
import json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
metadata=json.loads(subprocess.check_output(["cargo","metadata","--locked","--format-version","1"],cwd=ROOT))
parts=["7mlabs Numerology SDK 1.0.0 — third-party notices\nThe SDK's own code is MIT, Copyright (c) 2026 7mlabs.\nThe following dependency licenses and copyright notices are retained.\n"]
for package in sorted(metadata["packages"],key=lambda p:(p["name"],p["version"])):
 if package["source"] is None:continue
 directory=Path(package["manifest_path"]).parent
 candidates=[p for p in directory.iterdir() if p.is_file() and p.name.upper().startswith(("LICENSE","COPYING","NOTICE","COPYRIGHT"))]
 if not candidates:
  sources=json.loads((ROOT/"third-party/license-sources.json").read_text())
  record=sources[package["name"]]
  if record["version"]!=package["version"]:raise SystemExit("Upstream notice version differs")
  candidates=[ROOT/record["licenseFile"]]
 parts.append(f"\n{'='*72}\n{package['name']} {package['version']} — {package['license']}\n")
 for p in sorted(candidates):parts.append(f"\n--- {p.name} ---\n"+p.read_text())
(ROOT/"THIRD-PARTY-NOTICES.txt").write_text("\n".join(parts))
print(f"Collected notices for {sum(p['source'] is not None for p in metadata['packages'])} registry dependencies")
