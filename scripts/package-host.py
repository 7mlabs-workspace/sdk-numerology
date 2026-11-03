#!/usr/bin/env python3
"""Create guarded host-only candidates. No registry upload or credentials."""
import hashlib,json,platform,subprocess,shutil,sys,os
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'artifacts/packages';OUT.mkdir(parents=True,exist_ok=True)
for pattern in ['*.whl','*.tgz','*.nupkg','*-manifest.json','consumer-verification.json']:
 for old in OUT.glob(pattern):old.unlink()
if platform.system()=='Darwin':os.environ['MACOSX_DEPLOYMENT_TARGET']='11.0'
# A release staging directory is explicit and isolated from the guarded source package.
stage=ROOT/'artifacts/npm-stage';stage.mkdir(exist_ok=True)
for name in ['index.cjs','index.d.ts','LICENSE','THIRD-PARTY-NOTICES.txt','README.md']:shutil.copy2(ROOT/'bindings/node'/name,stage/name)
shutil.copytree(ROOT/'bindings/node/native',stage/'native',dirs_exist_ok=True)
meta=json.loads((ROOT/'bindings/node/package.json').read_text());assert meta.pop('private') is True;meta['publishConfig']={'access':'public','tag':'latest'}
(stage/'package.json').write_text(json.dumps(meta,indent=2)+'\n')
subprocess.run(['npm','pack','--ignore-scripts','--pack-destination',str(OUT)],cwd=stage,check=True)
subprocess.run([sys.executable,'-m','build','--no-isolation','--wheel','--outdir',str(OUT)],cwd=ROOT/'bindings/python',check=True)
if platform.system()=='Linux':
 wheel=next(OUT.glob('*.whl'));repair=ROOT/'artifacts/repaired';repair.mkdir(exist_ok=True)
 subprocess.run([sys.executable,'-m','auditwheel','repair','--plat','manylinux_2_35_x86_64','--wheel-dir',str(repair),str(wheel)],check=True)
 fixed=list(repair.glob('*.whl'));assert len(fixed)==1
 wheel.unlink();shutil.copy2(fixed[0],OUT/fixed[0].name)
subprocess.run(['dotnet','pack',str(ROOT/'bindings/dotnet/SevenMLabs.Numerology/SevenMLabs.Numerology.csproj'),'-c','Release','-o',str(OUT)],check=True)
files=[p for p in OUT.iterdir() if p.is_file()]
manifest={'version':'1.0.0','platform':platform.system(),'machine':platform.machine(),'candidateOnly':True,'artifacts':[{'name':p.name,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size} for p in sorted(files)]}
(OUT/'host-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');print(json.dumps(manifest))
