import os,platform,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];venv=ROOT/'artifacts/py310'
subprocess.run([sys.executable,'-m','venv',str(venv)],check=True)
wheels=[p for p in (ROOT/'artifacts/packages').glob('*.whl') if ('macosx' in p.name)==(platform.system()=='Darwin')];assert len(wheels)==1
python=venv/'bin/python';subprocess.run([str(python),'-m','pip','install','--no-index',str(wheels[0])],check=True)
subprocess.run([str(python),str(ROOT/'scripts/test-python.py')],env={**os.environ,'NUMEROLOGY_INSTALLED_PACKAGE':'1'},check=True)
