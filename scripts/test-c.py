#!/usr/bin/env python3
"""Compile a real C consumer against the public header and current host library."""
import subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
binary = ROOT / 'artifacts/c-consumer'
binary.parent.mkdir(exist_ok=True)
lib = ROOT / 'target/release'
subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-I', str(ROOT/'include'),
                str(ROOT/'tests/c/consumer.c'), '-L', str(lib), '-lsevenmlabs_numerology',
                '-Wl,-rpath,'+str(lib), '-o', str(binary)], check=True)
subprocess.run([str(binary)], check=True)
