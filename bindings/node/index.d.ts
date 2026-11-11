export type LegacyOperation = 'profile' | 'name' | 'birth' | 'cycles' | 'essence';
export type Operation = LegacyOperation;
export type SupportedOperation = LegacyOperation | 'names';
export interface PersonName { given: string; middle?: string; family?: string; }
export interface Options { keepMasterNumbers?: boolean; yRule?: 'contextual' | 'vowel' | 'consonant'; }
export interface ExtensionOptions { reducedValues?: boolean; indicators?: boolean; }
export interface NameExtensionOptions extends ExtensionOptions { currentName?: PersonName | null; nameChallenges?: NameChallengeOptions | null; }
export interface ProfileExtensionOptions extends NameExtensionOptions { nameSet?: NameSet | null; }
export interface CommonRequest { system?: 'pythagorean'; options?: Options; }
export type LegacyRequest = CommonRequest & (
 { operation: 'name'; name: PersonName; extensions?: NameExtensionOptions | null } |
 { operation: 'birth'; birthDate: string; extensions?: ExtensionOptions | null } |
 { operation: 'cycles'; birthDate: string; referenceDate: string; extensions?: ExtensionOptions | null } |
 { operation: 'essence'; name: PersonName; birthDate: string; startAge?: number; years?: number; extensions?: ExtensionOptions | null } |
 { operation: 'profile'; name: PersonName; birthDate: string; referenceDate: string; startAge?: number; years?: number; extensions?: ProfileExtensionOptions | null }
);
export interface CalculatedNumber { root: number; value: number; steps: number[]; isMaster: boolean; karmicDebt: number | null; reducedValue?: number; }
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
export interface Result<O extends LegacyOperation = LegacyOperation> { schemaVersion: '1.0'; engineVersion: '1.0.0'; system: 'pythagorean'; rulesVersion: 'pythagorean-7mlabs-1.0'; operation: O; options: Required<Options>; data: DataMap[O]; warnings: string[]; }
export interface GridAxis { digits: [number, number, number]; status: 'present' | 'absent' | 'partial'; }
export interface PeriodCycle extends Pinnacle { source: 'birthMonth' | 'birthDay' | 'birthYear'; }
export interface ExtendedIndicators {
 birthGridAxes?: GridAxis[];
 birthNameGridAxes?: GridAxis[];
 periodCycles?: PeriodCycle[];
 rationalThought?: { nameSource: 'birthName.given'; number: CalculatedNumber };
 currentName?: NameNumbers;
 nameSet?: NameSetResult;
 nameChallenges?: { birthName: NameChallengeResult; currentName?: NameChallengeResult };
 combinedGrid?: { nameSource: 'birthName' | 'currentName'; nameFrequency: Record<string, number>; frequency: Record<string, number>; axes: GridAxis[] };
}
export interface ExtendedResult<O extends LegacyOperation = LegacyOperation> extends Omit<Result<O>, 'schemaVersion' | 'data'> {
 schemaVersion: '1.1';
 extensions: { rulesVersion: 'pythagorean-7mlabs-extensions-1.0'; reducedValues: boolean; indicators: boolean; namesRulesVersion?: 'pythagorean-7mlabs-names-1.0'; nameChallengesRulesVersion?: 'pythagorean-7mlabs-name-challenges-1.0' };
 data: DataMap[O] & { extended?: ExtendedIndicators };
}
export interface CalculationError { code: string; field: string; message: string; }
export class NumerologyError extends Error { readonly code: string; readonly field: string; }
export interface NameInput { id: string; name: PersonName; }
export type NamePart = 'full' | 'given' | 'middle' | 'family';
export interface NameSource { nameId: string; part?: NamePart; }
export interface NameCombination { id: string; sources: readonly NameSource[]; }
export interface NameSet { names?: readonly NameInput[]; combinations?: readonly NameCombination[]; }
export interface NamesRequest extends CommonRequest { operation: 'names'; names: readonly NameInput[]; combinations?: readonly NameCombination[]; birthDate?: string; }
export type Request = LegacyRequest;
export type AnyRequest = LegacyRequest | NamesRequest;
export interface SingleNameResult { id: string; source: 'input' | 'profile.birthName'; numbers: NameNumbers; rationalThought?: { nameSource: 'given'; number: CalculatedNumber }; }
export interface NameCombinationResult { id: string; method: 'concatenateSelectedParts'; reduction: 'wholeNameSum'; sources: Array<Required<NameSource> & { text: string }>; numbers: NameNumbers; rationalThought?: { nameSource: 'selectedText'; number: CalculatedNumber }; }
export interface NameSetResult { rulesVersion: 'pythagorean-7mlabs-names-1.0'; names: SingleNameResult[]; combinations: NameCombinationResult[]; }
export interface NamesResult { schemaVersion: '1.1'; engineVersion: '1.0.0'; system: 'pythagorean'; rulesVersion: 'pythagorean-7mlabs-1.0'; namesRulesVersion: 'pythagorean-7mlabs-names-1.0'; operation: 'names'; options: Required<Options>; data: { names: NameSetResult }; warnings: string[]; }
export type CalculationResult<R extends AnyRequest> = R extends NamesRequest ? NamesResult : R extends LegacyRequest ? Result<R['operation']> | ExtendedResult<R['operation']> : never;
export function calculate<R extends NamesRequest>(request: R): NamesResult;
export function calculate<R extends LegacyRequest & { extensions: ExtensionOptions }>(request: R): ExtendedResult<R['operation']>;
export function calculate<R extends LegacyRequest & { extensions?: null | undefined }>(request: R): Result<R['operation']>;
export function calculate<R extends AnyRequest>(request: R): CalculationResult<R>;
/** Raw JSON envelope, including structured calculation errors. */
export function calculateJson(request: string): string;
export const version: '1.0.0';

export interface NameChallengeOptions { nameOrder: 'givenMiddleFamily' | 'familyMiddleGiven'; }
export type LetterChallenge =
 { status: 'available'; first: { letter: string; value: number }; last: { letter: string; value: number }; difference: number; value: number } |
 { status: 'unavailable'; reason: 'noMatchingLetters'; first: null; last: null; difference: null; value: null };
export interface NameChallengeResult {
 rulesVersion: 'pythagorean-7mlabs-name-challenges-1.0'; nameOrder: NameChallengeOptions['nameOrder']; orderedText: string; yRule: Required<Options>['yRule'];
 soulUrgeMethod: 'firstLastVowelAbsoluteDifference'; soulUrgeZeroPolicy: 'retainZero'; soulUrge: LetterChallenge;
 personalityMethod: 'firstLastConsonantAbsoluteDifference'; personalityZeroPolicy: 'mapZeroToNine'; personality: LetterChallenge;
}
