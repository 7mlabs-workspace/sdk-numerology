"use strict";
const path = require('node:path');
const binary=path.join(__dirname, 'native', `${process.platform}-${process.arch}`, 'numerology.node');
if (!require('node:fs').existsSync(binary)) throw new Error(`No bundled Numerology binary for ${process.platform}/${process.arch}; see release platform requirements`);
const native = require(binary);
if (native.engineVersion() !== '1.1.0') throw new Error('Numerology native engine version differs from wrapper');
function calculateJson(request) {
 if (typeof request !== 'string') throw new TypeError('calculateJson requires a JSON string');
 return native.calculateJson(request);
}
function calculateFilteredJson(request) {
 if (typeof request !== 'string') throw new TypeError('calculateFilteredJson requires a JSON string');
 return native.calculateFilteredJson(request);
}
class NumerologyError extends Error {
 constructor(error) { super(error.message); this.name='NumerologyError'; this.code=error.code; this.field=error.field; }
}
function calculate(request) {
 const envelope=JSON.parse(calculateJson(JSON.stringify(request)));
 if (!envelope.ok) throw new NumerologyError(envelope.error);
 return envelope.result;
}
function calculateFiltered(request) {
 const envelope=JSON.parse(calculateFilteredJson(JSON.stringify(request)));
 if (!envelope.ok) throw new NumerologyError(envelope.error);
 return envelope.result;
}
module.exports={calculate,calculateJson,calculateFiltered,calculateFilteredJson,NumerologyError,version:'1.1.0'};
