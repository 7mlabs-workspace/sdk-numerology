#!/usr/bin/env python3
"""Install candidates in isolated consumers and verify their actual native payloads."""
import hashlib,json,os,subprocess,sys,tarfile,zipfile,shutil,platform
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];out=ROOT/'artifacts/packages';consumers=ROOT/'artifacts/consumers';consumers.mkdir(parents=True,exist_ok=True)
manifest=json.loads((out/'host-manifest.json').read_text())
for record in manifest['artifacts']:
 p=out/record['name'];assert hashlib.sha256(p.read_bytes()).hexdigest()==record['sha256']
 if p.suffix=='.tgz':
  with tarfile.open(p) as t:
   names=t.getnames();assert 'package/LICENSE' in names and 'package/THIRD-PARTY-NOTICES.txt' in names
   meta=json.load(t.extractfile('package/package.json'));assert meta['version']=='1.0.0' and meta['license']=='MIT' and not meta.get('private');assert any(n.endswith('.node') for n in names)
 elif p.suffix in ['.whl','.nupkg']:
  with zipfile.ZipFile(p) as z:
   names=z.namelist();assert any(n.endswith('LICENSE') for n in names);assert any(n.endswith('THIRD-PARTY-NOTICES.txt') for n in names);assert any(n.endswith(('.so','.dylib')) for n in names)
node=consumers/'node';node.mkdir(exist_ok=True);(node/'package.json').write_text('{"private":true}')
tgz=next(out.glob('*.tgz'));subprocess.run(['npm','install','--ignore-scripts','--no-audit','--no-fund',str(tgz)],cwd=node,check=True)
subprocess.run(['node',str(ROOT/'scripts/test-node.cjs')],env={**os.environ,'NUMEROLOGY_NODE_PACKAGE':str(node/'node_modules/@7mlabs/numerology')},check=True)
venv=consumers/'python';subprocess.run([sys.executable,'-m','venv',str(venv)],check=True);python=venv/'bin/python'
wheels=[p for p in out.glob('*.whl') if ('macosx' in p.name)==(platform.system()=='Darwin')];assert len(wheels)==1;wheel=wheels[0];subprocess.run([str(python),'-m','pip','install','--no-index','--force-reinstall',str(wheel)],check=True)
subprocess.run([str(python),str(ROOT/'scripts/test-python.py')],env={**os.environ,'NUMEROLOGY_INSTALLED_PACKAGE':'1'},check=True)
dotnet=consumers/'dotnet';dotnet.mkdir(exist_ok=True)
(dotnet/'Consumer.csproj').write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings></PropertyGroup><ItemGroup><PackageReference Include="SevenMLabs.Numerology" Version="1.0.0" /></ItemGroup></Project>')
(dotnet/'NuGet.Config').write_text(f'<configuration><packageSources><clear/><add key="local" value="{out}" /></packageSources></configuration>')
shutil.copy2(ROOT/'tests/dotnet/Program.cs',dotnet/'Program.cs')
subprocess.run(['dotnet','run','--project',str(dotnet/'Consumer.csproj'),'--',str(ROOT/'artifacts/conformance.json')],env={**os.environ,'NUGET_PACKAGES':str(consumers/'nuget-cache')},check=True)
(out/'consumer-verification.json').write_text(json.dumps({'status':'passed','sharedCases':len(json.loads((ROOT/'artifacts/conformance.json').read_text())),'consumers':['installed npm tarball','installed Python wheel','installed NuGet package'],'manifestSha256':hashlib.sha256((out/'host-manifest.json').read_bytes()).hexdigest()},indent=2)+'\n')
print('All actual installed package consumers passed.')
