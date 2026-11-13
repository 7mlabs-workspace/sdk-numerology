const fs=require('node:fs');const assert=require('node:assert/strict');const path=require('node:path');
const sdk=require(process.env.NUMEROLOGY_NODE_PACKAGE||path.join(__dirname,'../bindings/node'));
const cases=JSON.parse(fs.readFileSync(process.argv[2]||path.join(__dirname,'../artifacts/conformance.json'),'utf8'));
for (const test of cases) assert.deepEqual(JSON.parse(sdk.calculateJson(test.request)),test.expected);
const filteredCases=JSON.parse(fs.readFileSync(path.join(__dirname,'../artifacts/conformance-filtered.json'),'utf8'));
for (const test of filteredCases) {
 assert.deepEqual(JSON.parse(sdk.calculateFilteredJson(test.request)),test.expected);
 if(test.expected.ok) assert.deepEqual(sdk.calculateFiltered(JSON.parse(test.request)),test.expected.result);
}
const request={operation:'name',name:{given:'Đặng An'}};const before=JSON.stringify(request);sdk.calculate(request);assert.equal(JSON.stringify(request),before);
const namesRequest={operation:'names',birthDate:'2000-02-29',names:[{id:'a',name:{given:'Ada'}},{id:'nick',name:{given:'Map'}}],combinations:[{id:'selected',sources:[{nameId:'a'},{nameId:'nick'}]}]};
const namesBefore=JSON.stringify(namesRequest);const namesResult=sdk.calculate(namesRequest);assert.equal(JSON.stringify(namesRequest),namesBefore);assert.equal(namesResult.data.names.names.length,2);assert.equal(namesResult.data.names.combinations.length,1);
assert.throws(()=>sdk.calculate({operation:'names',names:[]}),e=>e.code==='INVALID_RANGE');
assert.throws(()=>sdk.calculate({operation:'name',name:{given:'123'}}),e=>e.code==='UNSUPPORTED_CHARACTER');
assert.throws(()=>sdk.calculateJson({}),TypeError);
assert.throws(()=>sdk.calculateFilteredJson({}),TypeError);
assert.throws(()=>sdk.calculateFiltered({operation:'name',name:{given:'123'}}),e=>e.code==='UNSUPPORTED_CHARACTER');
const filteredBefore=JSON.stringify(namesRequest);sdk.calculateFiltered(namesRequest);assert.equal(JSON.stringify(namesRequest),filteredBefore);
const challengeRequest={operation:'name',name:{given:'Steve',family:'Jones'},extensions:{nameChallenges:{nameOrder:'givenMiddleFamily'}}};
const challengeBefore=JSON.stringify(challengeRequest);const challenges=sdk.calculate(challengeRequest).data.extended.nameChallenges.birthName;
assert.equal(JSON.stringify(challengeRequest),challengeBefore);assert.equal(challenges.personality.difference,0);assert.equal(challenges.personality.value,9);assert.equal(challenges.soulUrge.value,0);
for(let i=0;i<5000;i++)sdk.calculate(i%2?namesRequest:{operation:'birth',birthDate:'2000-02-29'});
for(let i=0;i<1000;i++)sdk.calculateFiltered(namesRequest);
console.log(JSON.stringify({runtime:process.version,cases:cases.length,filteredCases:filteredCases.length,repeatedCalculations:5000,repeatedFilteredCalculations:1000,repeatedNamesCalculations:2500,status:'passed'}));
