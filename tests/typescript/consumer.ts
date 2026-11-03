import { calculate, calculateJson, NumerologyError, Request } from '../../bindings/node';
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
