const fs=require('node:fs');const assert=require('node:assert/strict');const path=require('node:path');
const sdk=require(process.env.NUMEROLOGY_NODE_PACKAGE||path.join(__dirname,'../bindings/node'));
const cases=JSON.parse(fs.readFileSync(process.argv[2]||path.join(__dirname,'../artifacts/conformance.json'),'utf8'));
for (const test of cases) assert.deepEqual(JSON.parse(sdk.calculateJson(test.request)),test.expected);
const request={operation:'name',name:{given:'Đặng An'}};const before=JSON.stringify(request);sdk.calculate(request);assert.equal(JSON.stringify(request),before);
assert.throws(()=>sdk.calculate({operation:'name',name:{given:'123'}}),e=>e.code==='UNSUPPORTED_CHARACTER');
assert.throws(()=>sdk.calculateJson({}),TypeError);
for(let i=0;i<5000;i++)sdk.calculate({operation:'birth',birthDate:'2000-02-29'});
console.log(JSON.stringify({runtime:process.version,cases:cases.length,repeatedCalculations:5000,status:'passed'}));
