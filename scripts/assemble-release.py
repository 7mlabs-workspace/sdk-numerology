#!/usr/bin/env python3
"""Merge only validated native targets from the same immutable source commit."""
import hashlib,json,os,shutil,subprocess,tarfile,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];inputs=ROOT/'artifacts/download';out=ROOT/'artifacts/release';out.mkdir(parents=True,exist_ok=True)
sha=os.environ['GITHUB_SHA'];repository='7mlabs-workspace/sdk-numerology';provenances=[];conformance=None
for target,rid in [('darwin-arm64','osx-arm64'),('linux-x64','linux-x64')]:
 base=inputs/('native-'+target);proof=json.loads((base/'provenance.json').read_text());assert proof['commit']==sha and proof['repository']==repository and proof['engineVersion']=='1.0.0'
 for rel,expected in proof['files'].items():
  file=base/rel;assert file.resolve().is_relative_to(base.resolve());assert hashlib.sha256(file.read_bytes()).hexdigest()==expected,rel
 verified=json.loads((base/'packages/consumer-verification.json').read_text());assert verified['status']=='passed'
 assert proof['conformance']['cases']>0 and verified['sharedCases']==proof['conformance']['cases']
 assert verified['conformanceSha256']==proof['conformance']['sha256']
 if conformance is None:conformance=proof['conformance']
 else:assert conformance==proof['conformance'], 'Native targets must test the same contract fixtures'
 assert verified['manifestSha256']==hashlib.sha256((base/'packages/host-manifest.json').read_bytes()).hexdigest()
 candidate=json.loads((base/'packages/host-manifest.json').read_text())
 for artifact in candidate['artifacts']:assert hashlib.sha256((base/'packages'/artifact['name']).read_bytes()).hexdigest()==artifact['sha256']
 # Compare portable JS/type/notice files to this exact checkout.
 with tarfile.open(next((base/'packages').glob('*.tgz'))) as t:
  for rel in ['index.cjs','index.d.ts','LICENSE','THIRD-PARTY-NOTICES.txt','README.md']:assert t.extractfile('package/'+rel).read()==(ROOT/'bindings/node'/rel).read_bytes(),rel
 shutil.copytree(base/'node',ROOT/'bindings/node/native',dirs_exist_ok=True)
 shutil.copytree(base/'dotnet',ROOT/'bindings/dotnet/SevenMLabs.Numerology/runtimes',dirs_exist_ok=True)
 wheels=list((base/'packages').glob('*.whl'));assert len(wheels)==1
 shutil.copy2(wheels[0],out/wheels[0].name)
 provenances.append(proof)
# Final npm contains exactly the two tested native targets and no install script.
stage=ROOT/'artifacts/final-npm-stage';stage.mkdir(exist_ok=True)
for name in ['index.cjs','index.d.ts','LICENSE','THIRD-PARTY-NOTICES.txt','README.md']:shutil.copy2(ROOT/'bindings/node'/name,stage/name)
shutil.copytree(ROOT/'bindings/node/native',stage/'native',dirs_exist_ok=True)
assert sorted(p.name for p in (stage/'native').iterdir())==['darwin-arm64','linux-x64']
package=json.loads((ROOT/'bindings/node/package.json').read_text());assert package.pop('private') is True;package['publishConfig']={'access':'public','tag':'latest'};(stage/'package.json').write_text(json.dumps(package,indent=2)+'\n')
subprocess.run(['npm','pack','--ignore-scripts','--pack-destination',str(out)],cwd=stage,check=True)
subprocess.run(['dotnet','pack',str(ROOT/'bindings/dotnet/SevenMLabs.Numerology/SevenMLabs.Numerology.csproj'),'-c','Release','-o',str(out),'-p:RepositoryCommit='+sha,'-p:ContinuousIntegrationBuild=true'],check=True)
# Bind native payloads in final archives to their tested bytes.
with tarfile.open(next(out.glob('*.tgz'))) as t:
 for target in ['darwin-arm64','linux-x64']:
  file=ROOT/'bindings/node/native'/target/'numerology.node';assert t.extractfile('package/native/'+target+'/numerology.node').read()==file.read_bytes()
with zipfile.ZipFile(next(out.glob('*.nupkg'))) as z:
 for rid in ['osx-arm64','linux-x64']:
  ext='dylib' if rid.startswith('osx') else 'so';name=f'runtimes/{rid}/native/libsevenmlabs_numerology.{ext}'
  assert z.read(name)==(ROOT/'bindings/dotnet/SevenMLabs.Numerology'/name).read_bytes()
artifacts=[{'name':p.name,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size} for p in sorted(out.iterdir()) if p.suffix in ['.whl','.tgz','.nupkg']];assert len(artifacts)==4
manifest={'version':'1.0.0','sourceCommit':sha,'repository':repository,'license':'MIT','copyright':'Copyright (c) 2026 7mlabs','nativeTargets':['darwin-arm64','linux-x64'],'python':'3.10+','dotnet':'8.0+','node':'18+','rulesVersion':'pythagorean-7mlabs-1.0','artifacts':artifacts,'candidateOnly':True,'conformance':conformance}
(out/'release-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');(out/'host-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
(out/'SHA256SUMS').write_text(''.join(f"{p['sha256']}  {p['name']}\n" for p in artifacts))
print(json.dumps(manifest))
