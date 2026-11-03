"""Verify immutable tested artifacts before the publisher job gets OIDC credentials."""
import hashlib,json,os,subprocess,shutil,zipfile,email
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];run_id=os.environ['VALIDATED_RUN_ID']
assert run_id.isascii() and run_id.isdecimal()
run=json.loads(subprocess.check_output(['gh','api',f'repos/7mlabs-workspace/sdk-numerology/actions/runs/{run_id}']))
assert run['conclusion']=='success' and run['status']=='completed' and run['head_branch']=='main' and run['workflow_id']==405298409
assert run['repository']['full_name']=='7mlabs-workspace/sdk-numerology'
sha=run['head_sha'];subprocess.run(['git','merge-base','--is-ancestor',sha,'HEAD'],cwd=ROOT,check=True)
changed=subprocess.check_output(['git','diff','--name-only',sha,'HEAD'],cwd=ROOT,text=True).splitlines()
assert set(changed)<={'scripts/verify-publish.py','.github/workflows/publish-pypi.yml'},changed
if os.environ.get('VERIFY_MODE')=='identity':
 print('Verified successful SDK build '+sha)
else:
 base=ROOT/'artifacts/validated';manifest=json.loads((base/'release-manifest.json').read_text());assert manifest['sourceCommit']==sha and manifest['repository']=='7mlabs-workspace/sdk-numerology' and manifest['version']=='1.0.0'
 out=ROOT/'artifacts/pypi-dist';out.mkdir(parents=True,exist_ok=True);count=0
 for item in manifest['artifacts']:
  p=base/item['name'];assert p.resolve().is_relative_to(base.resolve());assert hashlib.sha256(p.read_bytes()).hexdigest()==item['sha256']
  if p.suffix!='.whl':continue
  with zipfile.ZipFile(p) as z:
   meta=email.message_from_bytes(z.read('sevenmlabs_numerology-1.0.0.dist-info/METADATA'))
   assert meta['Name']=='sevenmlabs-numerology' and meta['Version']=='1.0.0' and meta['License-Expression']=='MIT'
   assert b'Copyright (c) 2026 7mlabs' in z.read('sevenmlabs_numerology/LICENSE')
  shutil.copy2(p,out/p.name);count+=1
 assert count==2
 print(json.dumps({'verifiedSourceCommit':sha,'wheels':count,'project':'sevenmlabs-numerology'}))
