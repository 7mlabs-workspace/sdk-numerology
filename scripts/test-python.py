import copy, ctypes, gc, json, sys, os
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
if not os.environ.get('NUMEROLOGY_INSTALLED_PACKAGE'):sys.path.insert(0,str(ROOT/'bindings/python'))
import sevenmlabs_numerology as sdk
if os.environ.get("NUMEROLOGY_INSTALLED_PACKAGE"):assert Path(sdk.__file__).is_relative_to(Path(sys.prefix))
cases=json.loads((ROOT/'artifacts/conformance.json').read_text())
for test in cases:assert json.loads(sdk.calculate_json(test['request']))==test['expected']
filtered_cases=json.loads((ROOT/'artifacts/conformance-filtered.json').read_text())
for test in filtered_cases:
 assert json.loads(sdk.calculate_filtered_json(test['request']))==test['expected']
 if test['expected']['ok']:assert sdk.calculate_filtered(json.loads(test['request']))==test['expected']['result']
request={'operation':'name','name':{'given':'Đặng An'}};original=copy.deepcopy(request);sdk.calculate(request);assert request==original
names_request={'operation':'names','birthDate':'2000-02-29','names':[{'id':'a','name':{'given':'Ada'}},{'id':'nick','name':{'given':'Map'}}],'combinations':[{'id':'selected','sources':[{'nameId':'a'},{'nameId':'nick'}]}]}
names_before=copy.deepcopy(names_request);names_result=sdk.calculate(names_request);assert names_request==names_before
sdk.calculate_filtered(names_request);assert names_request==names_before
assert len(names_result['data']['names']['names'])==2 and len(names_result['data']['names']['combinations'])==1
try:sdk.calculate({'operation':'name','name':{'given':'123'}});raise AssertionError('error expected')
except sdk.NumerologyError as e:assert e.code=='UNSUPPORTED_CHARACTER'
try:sdk.calculate_filtered({'operation':'name','name':{'given':'123'}});raise AssertionError('error expected')
except sdk.NumerologyError as e:assert e.code=='UNSUPPORTED_CHARACTER'
challenge_request={'operation':'name','name':{'given':'Steve','family':'Jones'},'extensions':{'nameChallenges':{'nameOrder':'givenMiddleFamily'}}}
challenge_before=copy.deepcopy(challenge_request);challenges=sdk.calculate(challenge_request)['data']['extended']['nameChallenges']['birthName']
assert challenge_request==challenge_before
assert challenges['personality']['difference']==0 and challenges['personality']['value']==9 and challenges['soulUrge']['value']==0
for i in range(5000):sdk.calculate(names_request if i%2 else {'operation':'birth','birthDate':'2000-02-29'})
pointer=sdk._lib.numerology_calculate_json(None)
try:assert json.loads(ctypes.string_at(pointer))["error"]["code"]=="NULL_INPUT"
finally:sdk._lib.numerology_free(pointer)
pointer=sdk._lib.numerology_calculate_json(b'\xff')
try:assert json.loads(ctypes.string_at(pointer))['error']['code']=='INVALID_UTF8'
finally:sdk._lib.numerology_free(pointer)
sdk._lib.numerology_free(None)
for i in range(1000):sdk.calculate_filtered(names_request)
print(json.dumps({'python':sys.version.split()[0],'cases':len(cases),'filteredCases':len(filtered_cases),'repeatedCalculations':5000,'repeatedFilteredCalculations':1000,'repeatedNamesCalculations':2500,'status':'passed'}))
