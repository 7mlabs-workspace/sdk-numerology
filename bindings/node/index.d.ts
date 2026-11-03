export type Operation = 'profile' | 'name' | 'birth' | 'cycles' | 'essence';
export interface PersonName { given: string; middle?: string; family?: string; }
export interface Options { keepMasterNumbers?: boolean; yRule?: 'contextual' | 'vowel' | 'consonant'; }
export interface CommonRequest { system?: 'pythagorean'; options?: Options; }
export type Request = CommonRequest & (
 { operation: 'name'; name: PersonName } |
 { operation: 'birth'; birthDate: string } |
 { operation: 'cycles'; birthDate: string; referenceDate: string } |
 { operation: 'essence'; name: PersonName; birthDate: string; startAge?: number; years?: number } |
 { operation: 'profile'; name: PersonName; birthDate: string; referenceDate: string; startAge?: number; years?: number }
);
export interface CalculatedNumber { root: number; value: number; steps: number[]; isMaster: boolean; karmicDebt: number | null; }
export interface NameNumbers {
 normalizedName: Required<PersonName>; expression: CalculatedNumber; soulUrge: CalculatedNumber; personality: CalculatedNumber; balance: CalculatedNumber; givenName: CalculatedNumber; familyName: CalculatedNumber; frequency: Record<string, number>; missingNumbers: number[]; hiddenPassion: number[]; subconsciousSelf: number; cornerstone: string; capstone: string;
}
export interface BirthNumbers { lifePath: CalculatedNumber; birthDay: CalculatedNumber; attitude: CalculatedNumber; components: { day: CalculatedNumber; month: CalculatedNumber; year: CalculatedNumber }; birthGrid: Record<string, number>; }
export interface Pinnacle { phase: number; number: CalculatedNumber; startAge: number; endAge: number | null; }
export interface Cycles { personalYear: CalculatedNumber; personalMonth: CalculatedNumber; personalDay: CalculatedNumber; universalYear: CalculatedNumber; universalMonth: CalculatedNumber; universalDay: CalculatedNumber; pinnacles: Pinnacle[]; challenges: CalculatedNumber[]; }
export interface Transit { letter: string | null; value: number; }
export interface EssenceRow { age: number; birthYearPlusAge: number; given: Transit; middle: Transit; family: Transit; essence: CalculatedNumber; }
export interface Essence { years: EssenceRow[]; }
export interface Derived { maturity: CalculatedNumber; lifePathExpressionBridge: number; soulPersonalityBridge: number; }
export interface Profile { name: NameNumbers; birth: BirthNumbers; cycles: Cycles; essence: Essence; derived: Derived; }
export interface DataMap { name: Pick<Profile,'name'>; birth: Pick<Profile,'birth'>; cycles: Pick<Profile,'cycles'>; essence: Pick<Profile,'essence'>; profile: Profile; }
export interface Result<O extends Operation = Operation> { schemaVersion: '1.0'; engineVersion: '1.0.0'; system: 'pythagorean'; rulesVersion: 'pythagorean-7mlabs-1.0'; operation: O; options: Required<Options>; data: DataMap[O]; warnings: string[]; }
export interface CalculationError { code: string; field: string; message: string; }
export class NumerologyError extends Error { readonly code: string; readonly field: string; }
export function calculate<R extends Request>(request: R): Result<R['operation']>;
/** Raw JSON envelope, including structured calculation errors. */
export function calculateJson(request: string): string;
export const version: '1.0.0';
