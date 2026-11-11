#!/usr/bin/env python3
"""Compare the new CLI against frozen pre-extension responses, never regenerate them."""
import argparse
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--baseline', type=Path, default=ROOT/'tests/fixtures/legacy-contract.json')
parser.add_argument('--cli', type=Path, default=ROOT/'target/release/sevenmlabs-numerology-cli')
args = parser.parse_args()
baseline = json.loads(args.baseline.read_text())
cases = baseline['cases'] if isinstance(baseline, dict) else baseline
for i, case in enumerate(cases):
    response = subprocess.run([str(args.cli)], input=case['request'], text=True,
                              capture_output=True, check=True, timeout=5)
    actual = json.loads(response.stdout)
    if actual != case['expected']:
        raise SystemExit(f'Legacy contract changed at case {i}')
print(json.dumps({'status':'passed','legacyCases':len(cases),'comparison':'exact JSON values and fields'}))
