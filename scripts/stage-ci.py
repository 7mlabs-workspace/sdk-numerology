import json,hashlib,shutil,platform,os
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];out=ROOT/'artifacts/ci';out.mkdir(exist_ok=True)
for source,target in [('bindings/node/native','node'),('bindings/python/sevenmlabs_numerology/native','python'),('bindings/dotnet/SevenMLabs.Numerology/runtimes','dotnet'),('artifacts/packages','packages')]:shutil.copytree(ROOT/source,out/target,dirs_exist_ok=True)
# Installed consumer folders are intentionally excluded.
manifest={'commit':os.environ['GITHUB_SHA'],'repository':os.environ['GITHUB_REPOSITORY'],'engineVersion':'1.1.0','conformance':{'cases':len(json.loads((ROOT/'artifacts/conformance.json').read_text())),'sha256':hashlib.sha256((ROOT/'artifacts/conformance.json').read_bytes()).hexdigest()},'filteredConformance':{'cases':len(json.loads((ROOT/'artifacts/conformance-filtered.json').read_text())),'sha256':hashlib.sha256((ROOT/'artifacts/conformance-filtered.json').read_bytes()).hexdigest()},'system':platform.system(),'machine':platform.machine(),'files':{str(p.relative_to(out)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(out.rglob('*')) if p.is_file()}}
(out/'provenance.json').write_text(json.dumps(manifest,indent=2)+'\n')
