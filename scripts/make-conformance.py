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
# Exercise new payloads through every native consumer without altering legacy cases.
for op in ["profile","name","birth","cycles","essence"]:
 for ext in [{}, {"indicators":False}, {"reducedValues":False}, {"indicators":False,"reducedValues":False}]:
  requests.append(json.dumps({"operation":op,"name":{"given":"Ada","family":"Lovelace"},"birthDate":"1815-12-10","referenceDate":"2026-11-11","years":1,"extensions":ext}))
for current in [{"given":"Ada"},{"given":"Đặng Ly"},{"given":"123"}]:
 requests.append(json.dumps({"operation":"profile","name":{"given":"Ada"},"birthDate":"2000-02-29","referenceDate":"2026-01-01","years":1,"extensions":{"currentName":current}},ensure_ascii=False))
for raw in ['{"operation":"name","name":{"given":"A"},"extensions":{"indicators":true,"indicators":false}}','{"operation":"name","name":{"given":"A"},"extensions":{"typo":true}}','{"operation":"birth","birthDate":"2000-01-01","extensions":{"currentName":{"given":"A"}}}']:
 requests.append(raw)
# Multiple-name calculations and profile orchestration share the existing name core.
base_names=[{"id":"legal","name":{"given":"Ada-Yay","family":"Đặng"}},{"id":"nick","name":{"given":"Map"}}]
groups=[{"id":"given-nick","sources":[{"nameId":"legal","part":"given"},{"nameId":"nick"}]}]
for y in ["contextual","vowel","consonant"]:
 for keep in [True,False]:
  requests.append(json.dumps({"operation":"names","names":base_names,"combinations":groups,"birthDate":"2000-02-29","options":{"yRule":y,"keepMasterNumbers":keep}},ensure_ascii=False))
  requests.append(json.dumps({"operation":"profile","name":base_names[0]["name"],"birthDate":"2000-02-29","referenceDate":"2026-11-11","years":1,"options":{"yRule":y,"keepMasterNumbers":keep},"extensions":{"nameSet":{"names":[base_names[1]],"combinations":[{"id":"given-nick","sources":[{"nameId":"birth","part":"given"},{"nameId":"nick"}]}]}}},ensure_ascii=False))
requests.append(json.dumps({"operation":"names","names":base_names},ensure_ascii=False))
requests.append(json.dumps({"operation":"profile","name":{"given":"Ada"},"birthDate":"2000-02-29","referenceDate":"2026-11-11","years":1,"extensions":{"nameSet":{}}}))
for names,combinations in [
 ([{"id":"a","name":{"given":"An"}},{"id":"a","name":{"given":"Map"}}],[]),
 ([{"id":"a","name":{"given":"An"}}],[{"id":"g","sources":[{"nameId":"a"},{"nameId":"missing"}]}]),
 ([{"id":"a","name":{"given":"Án"}},{"id":"b","name":{"given":"An"}}],[{"id":"g","sources":[{"nameId":"a"},{"nameId":"b"}]}]),
 ([{"id":"a","name":{"given":"An"}},{"id":"b","name":{"given":"Map"}}],[{"id":"g","sources":[{"nameId":"a"},{"nameId":"b","part":"family"}]}]),
 ([{"id":"a","name":{"given":"An","family":"Dang"}}],[{"id":"g","sources":[{"nameId":"a"},{"nameId":"a","part":"given"}]}]),
 ([{"id":"a","name":{"given":"An"}}],[{"id":"g","sources":[{"nameId":"a"}]}]),
 ([{"id":" bad","name":{"given":"An"}}],[]),
 ([],[]),
]:
 requests.append(json.dumps({"operation":"names","names":names,"combinations":combinations},ensure_ascii=False))
for raw in ['{"operation":"names","names":null}','{"operation":"names","names":[],"names":[]}','{"operation":"names","names":[{"id":"a","name":{"given":"Ada"},"weight":2}]}']:
 requests.append(raw)
# Verified first/last-letter conventions: explicit order, zero mapping and absent classes.
for text in ["David Edward Hanson","Dave Jones","Steve Jones","A","B","AEIOU","BCDF","Ý-BY","YAEI"]:
 for order in ["givenMiddleFamily","familyMiddleGiven"]:
  for y in ["contextual","vowel","consonant"]:
   requests.append(json.dumps({"operation":"name","name":{"given":text},"options":{"yRule":y},"extensions":{"nameChallenges":{"nameOrder":order}}},ensure_ascii=False))
requests.append(json.dumps({"operation":"profile","name":{"given":"EON","middle":"RAI","family":"BU"},"birthDate":"2000-02-29","referenceDate":"2026-11-11","years":1,"extensions":{"nameChallenges":{"nameOrder":"familyMiddleGiven"},"currentName":{"given":"Steve Jones"},"nameSet":{"names":[{"id":"alias","name":{"given":"MAP"}}]}}}))
for settings in [{},{"nameOrder":"guess"},{"nameOrder":False},{"nameOrder":"givenMiddleFamily","extra":True}]:
 requests.append(json.dumps({"operation":"name","name":{"given":"A"},"extensions":{"nameChallenges":settings}}))
for op in ["birth","cycles","essence"]:
 requests.append(json.dumps({"operation":op,"name":{"given":"A"},"birthDate":"2000-01-01","referenceDate":"2026-01-01","extensions":{"nameChallenges":{"nameOrder":"givenMiddleFamily"}}}))
requests.append(json.dumps({"operation":"name","name":{"given":"A"},"extensions":{"indicators":False,"nameChallenges":{"nameOrder":"givenMiddleFamily"}}}))
requests.append(json.dumps({"operation":"name","name":{"given":"A"},"extensions":{"nameChallenges":None}}))
cli=ROOT/'target/release/sevenmlabs-numerology-cli';cases=[]
for request in requests:
 p=subprocess.run([str(cli)],input=request,text=True,capture_output=True,check=True,timeout=3)
 cases.append({"request":request,"expected":json.loads(p.stdout)})
path=ROOT/'artifacts/conformance.json';path.parent.mkdir(exist_ok=True);path.write_text(json.dumps(cases,ensure_ascii=False,separators=(',',':')))
print(json.dumps({"sharedCases":len(cases),"path":str(path)}))
