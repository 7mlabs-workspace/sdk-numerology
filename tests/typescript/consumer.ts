import { calculate, calculateJson, calculateFiltered, calculateFilteredJson, NumerologyError, Request, Operation, Result, AnyRequest, SupportedOperation } from '../../bindings/node';
const birth=calculate({operation:'birth',birthDate:'2000-02-29'});
const life: number=birth.data.birth.lifePath.value;
const profile=calculate({operation:'profile',name:{given:'An'},birthDate:'2000-02-29',referenceDate:'2026-01-01'});
const day: number=profile.data.cycles.personalDay.value;
const request: Request={operation:'name',name:{given:'Ly'},options:{yRule:'vowel'}};
calculate(request).data.name.soulUrge;
// @ts-expect-error cycle calculations require explicit referenceDate
calculate({operation:'cycles',birthDate:'2000-01-01'});
// @ts-expect-error this release supports Pythagorean only
calculate({operation:'birth',system:'chaldean',birthDate:'2000-01-01'});
// @ts-expect-error birth-only results have no name section
birth.data.name.expression;
calculateJson('{}'); void [life,day,NumerologyError];
const oldSchema: '1.0' = birth.schemaVersion;
const expanded = calculate({operation:'profile', name:{given:'Ada'}, birthDate:'1815-12-10', referenceDate:'2026-01-01', extensions:{currentName:{given:'Ada'}}});
const newSchema: '1.1' = expanded.schemaVersion;
const reduced: number | undefined = expanded.data.name.soulUrge.reducedValue;
const thought: number | undefined = expanded.data.extended?.rationalThought?.number.value;
const onlyReduced = calculate({operation:'birth',birthDate:'2000-02-29',extensions:{indicators:false}});
onlyReduced.data.birth.lifePath.reducedValue;
// @ts-expect-error extension switches are booleans
calculate({operation:'birth',birthDate:'2000-02-29',extensions:{reducedValues:'yes'}});
void [oldSchema,newSchema,reduced,thought];
const many = calculate({operation:'names',birthDate:'2000-02-29',names:[{id:'birth',name:{given:'Ada',family:'Lovelace'}},{id:'nick',name:{given:'Map'}}],combinations:[{id:'selected',sources:[{nameId:'birth',part:'given'},{nameId:'nick'}]}]});
const manySchema: '1.1' = many.schemaVersion;
const manyId: string = many.data.names.names[0].id;
const selectedSource: string = many.data.names.combinations[0].sources[0].nameId;
calculate({operation:'profile',name:{given:'Ada'},birthDate:'2000-02-29',referenceDate:'2026-11-11',extensions:{nameSet:{names:[{id:'nick',name:{given:'Map'}}]}}}).data.extended?.nameSet?.names;
// @ts-expect-error a names request requires its list
calculate({operation:'names'});
// @ts-expect-error source selection is limited to documented name parts
calculate({operation:'names',names:[{id:'a',name:{given:'Ada'}}],combinations:[{id:'g',sources:[{nameId:'a',part:'nickname'}]}]});
// @ts-expect-error a name set is a profile extension
calculate({operation:'name',name:{given:'Ada'},extensions:{nameSet:{}}});
// @ts-expect-error a standalone names result has no birth calculation section
many.data.birth.lifePath;
void [manySchema,manyId,selectedSource];
// Existing generic helpers constrained to Operation/Request still compile unchanged.
function oldGeneric<O extends Operation>(value: Result<O>): O { return value.operation; }
function oldRequestOperation<R extends Request>(value: R): Operation { return value.operation; }
const anyRequest: AnyRequest = {operation:'names',names:[{id:'a',name:{given:'Ada'}}]};
const supported: SupportedOperation = calculate(anyRequest).operation;
void [oldGeneric,oldRequestOperation,supported];

const challenged = calculate({operation:'name',name:{given:'Dave',family:'Jones'},extensions:{nameChallenges:{nameOrder:'givenMiddleFamily'}}});
const challenge = challenged.data.extended?.nameChallenges?.birthName.personality;
if (challenge?.status === 'available') { const v: number = challenge.value; const letter: string = challenge.first.letter; void [v,letter]; }
// @ts-expect-error order must be explicit
calculate({operation:'name',name:{given:'A'},extensions:{nameChallenges:{}}});
// @ts-expect-error birth calculations do not accept name challenges
calculate({operation:'birth',birthDate:'2000-01-01',extensions:{nameChallenges:{nameOrder:'givenMiddleFamily'}}});
const filtered = calculateFiltered({operation:'profile',name:{given:'Đặng'},birthDate:'2000-02-29',referenceDate:'2026-11-13'});
const filteredSchema: 'filtered-1.0' = filtered.schemaVersion;
const originalSum: number = filtered.data.name.expression.root;
const originalSteps: number[] = filtered.data.name.expression.steps;
const reducedDisplay: number = filtered.data.name.expression.reducedValue;
const withheld: 'withheld' = filtered.data.birth.attitude.traceStatus;
// @ts-expect-error filtered payload does not contain the normalized name
filtered.data.name.normalizedName;
// @ts-expect-error withheld traces cannot be accessed as ordinary traced numbers
filtered.data.birth.attitude.root;
// @ts-expect-error filtered birthday/source components are absent
filtered.data.birth.birthDay;
const filteredNames = calculateFiltered({operation:'names',names:[{id:'private',name:{given:'A'}}]});
const label: string = filteredNames.data.names.names[0].id;
// @ts-expect-error names results have no profile birth section
filteredNames.data.birth;
// @ts-expect-error invalid name source selection still fails for the new API
calculateFiltered({operation:'names',names:[{id:'a',name:{given:'A'}}],combinations:[{id:'b',sources:[{nameId:'a',part:'nickname'}]}]});
calculateFilteredJson('{}');
void [filteredSchema, originalSum, originalSteps, reducedDisplay, withheld, label];
