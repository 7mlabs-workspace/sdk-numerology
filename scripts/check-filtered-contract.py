#!/usr/bin/env python3
"""Validate shared filtered output against the closed schema and full-result arithmetic."""
import copy
import json
from pathlib import Path
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
schema = json.loads((ROOT / 'schemas/filtered-result.schema.json').read_text())
Draft202012Validator.check_schema(schema)
validator = Draft202012Validator(schema)
full = json.loads((ROOT / 'artifacts/conformance.json').read_text())
filtered = json.loads((ROOT / 'artifacts/conformance-filtered.json').read_text())
assert len(full) == len(filtered)
forbidden = {'normalizedName', 'cornerstone', 'capstone', 'orderedText', 'letter', 'first', 'last',
             'text', 'birthDate', 'components', 'birthDay', 'birthGrid', 'birthGridAxes',
             'combinedGrid', 'periodCycles', 'birthYearPlusAge'}
counts = {'cases': len(filtered), 'successes': 0, 'errors': 0, 'retainedTraces': 0,
          'withheldTraces': 0, 'negativeSchemaChecks': 0}
operations = set()

def digital_root(n):
    while n >= 10:
        n = sum(map(int, str(n)))
    return n

def compare(original, output, path='data'):
    if isinstance(output, dict):
        assert not forbidden.intersection(output), (path, forbidden.intersection(output))
        if 'traceStatus' in output:
            for key in ('value', 'isMaster', 'karmicDebt'):
                assert output[key] == original[key], (path, key)
            assert output['reducedValue'] == digital_root(original['value']), path
            if output['traceStatus'] == 'included':
                assert output['root'] == original['root'] and output['steps'] == original['steps'], path
                assert output['steps'][0] == output['root'] and output['steps'][-1] == output['value'], path
                counts['retainedTraces'] += 1
            else:
                assert output['traceStatus'] == 'withheld' and 'root' not in output and 'steps' not in output, path
                assert path.endswith('.attitude') or path.endswith('.rationalThought.number'), path
                counts['withheldTraces'] += 1
        else:
            for key, value in output.items():
                compare(original.get(key) if isinstance(original, dict) else None, value, path+'.'+key)
    elif isinstance(output, list):
        for i, value in enumerate(output):
            compare(original[i] if isinstance(original, list) and i < len(original) else None, value, path+f'[{i}]')

for old, new in zip(full, filtered):
    assert old['request'] == new['request']
    a, b = old['expected'], new['expected']
    assert a['ok'] == b['ok']
    if not b['ok']:
        assert a == b
        counts['errors'] += 1
        continue
    a, b = a['result'], b['result']
    validator.validate(b)
    counts['successes'] += 1
    operations.add(b['operation'])
    compare(a['data'], b['data'])
    assert a['options'] == b['options']
    for warning in b['warnings']:
        assert warning['code'] == 'CONTEXTUAL_Y_HEURISTIC'

assert operations == {'profile', 'name', 'birth', 'cycles', 'essence', 'names'}
# Negative checks prove that the schema also rejects unknown nested fields and stripped sources.
sample = next(case['expected']['result'] for case in filtered
              if case['expected'].get('ok') and case['expected']['result']['operation'] == 'profile')
def rejected(change):
    value = copy.deepcopy(sample)
    change(value)
    assert not validator.is_valid(value), 'Closed schema accepted a forbidden/invalid shape'
    counts['negativeSchemaChecks'] += 1

rejected(lambda x: x.update({'futurePersonalData': 'CANARY'}))
rejected(lambda x: x['data']['name'].update({'normalizedName': {'given': 'CANARY'}}))
rejected(lambda x: x['data']['name']['expression'].update({'futurePersonalData': 'CANARY'}))
rejected(lambda x: x['data']['name']['frequency'].update({'CANARY': 1}))
rejected(lambda x: x['data']['name']['expression'].pop('steps'))
rejected(lambda x: x['data']['birth'].update({'birthDay': {'value': 1}}))
rejected(lambda x: x['data']['birth']['attitude'].update({'root': 2, 'steps': [2]}))
rejected(lambda x: x['data']['essence']['years'][0].update({'birthYearPlusAge': 2000}))
counts['status'] = 'passed'
(ROOT / 'artifacts/filtered-verification.json').write_text(json.dumps(counts, indent=2)+'\n')
print(json.dumps(counts))
