#!/usr/bin/env python3
"""Generate shared native-consumer fixtures; independent arithmetic fixtures live in Rust tests."""
import json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
requests=[]
for birth in ["0001-01-01","1900-02-28","2000-02-29","1963-02-17","1975-11-25","2001-09-08","9999-12-31"]:
 for ref in ["2026-01-01","2026-12-31"]:
  for name in [{"given":"Ada","family":"Lovelace"},{"given":"Án","family":"Đặng"},{"given":"Ly"},{"given":"A-Yay","middle":"An","family":"O’Neil"}]:
   for op in ["profile","name","birth","cycles","essence"]:
    requests.append(json.dumps({"operation":op,"birthDate":birth,"referenceDate":ref,"name":name},ensure_ascii=False))
for name in ["12345","---","王明","", "A"*1025]:requests.append(json.dumps({"operation":"name","name":{"given":name}}))
for value in ["{}\0ignored", "", "null", "[]", "{}", '{"operation":"birth","operation":"name"}', '{"operation":"name","name":{"given":"Ada","extra":1}}', '{"operation":"name","name":{"given":"Ada"},"system":"chaldean"}', '{"operation":"birth","birthDate":"1900-02-29"}', '{"operation":"cycles","birthDate":"2000-01-01"}', ' '*16385]:requests.append(value)
for y in ["contextual","vowel","consonant"]:
 for keep in [True,False]:requests.append(json.dumps({"operation":"profile","name":{"given":"LY YAY"},"birthDate":"1963-02-17","referenceDate":"2026-02-03","options":{"yRule":y,"keepMasterNumbers":keep},"startAge":100,"years":21}))
cli=ROOT/'target/release/sevenmlabs-numerology-cli';cases=[]
for request in requests:
 p=subprocess.run([str(cli)],input=request,text=True,capture_output=True,check=True,timeout=3)
 cases.append({"request":request,"expected":json.loads(p.stdout)})
path=ROOT/'artifacts/conformance.json';path.parent.mkdir(exist_ok=True);path.write_text(json.dumps(cases,ensure_ascii=False,separators=(',',':')))
print(json.dumps({"sharedCases":len(cases),"path":str(path)}))
