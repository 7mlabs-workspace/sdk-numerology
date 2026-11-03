import copy, ctypes, gc, json, sys, os
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
if not os.environ.get('NUMEROLOGY_INSTALLED_PACKAGE'):sys.path.insert(0,str(ROOT/'bindings/python'))
import sevenmlabs_numerology as sdk
if os.environ.get("NUMEROLOGY_INSTALLED_PACKAGE"):assert Path(sdk.__file__).is_relative_to(Path(sys.prefix))
cases=json.loads((ROOT/'artifacts/conformance.json').read_text())
for test in cases:assert json.loads(sdk.calculate_json(test['request']))==test['expected']
request={'operation':'name','name':{'given':'Đặng An'}};original=copy.deepcopy(request);sdk.calculate(request);assert request==original
try:sdk.calculate({'operation':'name','name':{'given':'123'}});raise AssertionError('error expected')
except sdk.NumerologyError as e:assert e.code=='UNSUPPORTED_CHARACTER'
for _ in range(5000):sdk.calculate({'operation':'birth','birthDate':'2000-02-29'})
pointer=sdk._lib.numerology_calculate_json(None)
try:assert json.loads(ctypes.string_at(pointer))["error"]["code"]=="NULL_INPUT"
finally:sdk._lib.numerology_free(pointer)
pointer=sdk._lib.numerology_calculate_json(b'\xff')
try:assert json.loads(ctypes.string_at(pointer))['error']['code']=='INVALID_UTF8'
finally:sdk._lib.numerology_free(pointer)
sdk._lib.numerology_free(None)
print(json.dumps({'python':sys.version.split()[0],'cases':len(cases),'repeatedCalculations':5000,'status':'passed'}))
