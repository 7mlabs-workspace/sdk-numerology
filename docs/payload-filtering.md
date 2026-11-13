<a href="../README.en.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs full logo"></a>

# Filtered payload — SDK 1.1.0

[Tiếng Việt](payload-filtering.vi.md) · **English**

[SDK home](../README.en.md) · [Formulas](features-and-formulas.md) · [Rules](rules.md) · [Schema](../schemas/filtered-result.schema.json)

## Compatibility and scope

Version 1.1.0 is a release candidate, not published yet. All existing full-output APIs retain their request grammar, arithmetic, schema and errors. Only `engineVersion` changes with the package version. Vietnamese normalization and Pythagorean rules are unchanged.

New opt-in filtered APIs use the separate `filtered-1.0` schema and the same six operations. Existing extension switches still control which indicators are calculated. Every exported Number includes `reducedValue` even when `extensions.reducedValues=false`; that switch controls the full payload only.

Reduced disclosure **is not anonymization or permission to publish**. Numeric profiles, name frequencies and contextual information may still identify a person. `outputPolicy.anonymous` is always false. Applications must also control request/full-result logging, transmission and identifier mappings.

## Arithmetic traces

A retained expression total of 38 returns `root=38`, `steps=[38,11]`, `value=11`, `reducedValue=2`, `isMaster=true`. Disabling masters gives `steps=[38,11,2]`, `value=2`, `isMaster=false`. Zero remains zero; master displays 11/2, 22/4 and 33/6 remain available. Retained roots and steps exactly match the full calculation.

Attitude and every Rational Thought result are exceptions: retain value, reducedValue, isMaster and karmicDebt, but omit both root and steps. They carry `traceStatus="withheld"`, `traceWithheldReason="birthDateInference"`. Ordinary traced Numbers carry `traceStatus="included"`. No fake/null/zero trace is substituted. Thought total minus the given-name/selected-text total reveals the birth day; Attitude total minus that day reveals the month. Attitude total 2 alone identifies month/day 1/1.

## Field policy

| Full-output group | Filtered output |
| --- | --- |
| Name totals, frequencies, missing/hidden-passion/subconscious numbers | Retain arithmetic; omit normalizedName, cornerstone, capstone |
| Life Path | Retain Number and trace |
| Attitude | Retain result; withhold trace |
| birth.components, birth.birthDay, birth.birthGrid | Omit entire groups; even a final birthday value may equal the actual day |
| Personal/universal cycles, Pinnacles, birth Challenges | Retain numbers, traces, phase and age bands |
| extended.periodCycles, birthGridAxes, combinedGrid | Omit; these repeat birth sources or permit reconstructing the birth grid by subtraction |
| extended.birthNameGridAxes | Retain name-grid axes |
| Essence | Retain age and aggregate Essence Number/trace; omit birthYearPlusAge and all given/middle/family transit components, including encoded letter values |
| Name Challenges | Retain status/reason/difference/value and methods; omit orderedText and entire first/last objects, including numeric vowel encodings |
| Current names, name sets and combinations | Apply the same policy recursively; retain selected parts and references, omit source text |
| Caller IDs | Replace with n1/n2/c1… labels per name set; do not expose old IDs, mapping tables or identifying hashes |
| Warnings | Rebuild fixed code/field/message objects using schema paths and array offsets, never forward ID-containing full-output strings |
| Errors | Preserve validation, controlled codes/paths/messages; never echo the request or raw parser diagnostics |

Response labels are local to each name set, not stable identities or security tokens. The application keeps any necessary display mapping privately.

Cycle traces can still reconstruct the **reference date**, which is different from the birth date. `outputPolicy.referenceDateMayBeRecoverable` discloses this whenever data.cycles exists. Filtering does not promise resistance to every inference or linkage attack.

## APIs

```js
const { calculateFiltered } = require('@7mlabs/numerology');
const result = calculateFiltered({ operation: 'name', name: { given: 'Đặng' } });
console.log(result.data.name.expression); // root 17, steps [17,8], value 8
```

| Surface | New entry points |
| --- | --- |
| Rust | calculate_filtered(&Request), calculate_names_filtered(&NamesRequest), calculate_filtered_json, calculate_filtered_bytes; JSON supports all extensions |
| Node/TypeScript | calculateFiltered, calculateFilteredJson |
| Python | calculate_filtered, calculate_filtered_json |
| .NET | CalculateFiltered, CalculateFilteredJson |
| C ABI v1 | numerology_calculate_filtered_json_len, numerology_calculate_filtered_json; same numerology_free ownership |
| CLI | sevenmlabs-numerology-cli --filtered; JSON request on stdin |

Raw APIs keep the ok/result or ok/error envelope. TypeScript uses separate FilteredResult types; filtered data is not assignable to a full-output result.

The engine constructs output from explicit field allowlists. New full-output fields are not automatically exported. `calculationRules` describes effective normalization, name scope, Y and bridge conventions; each Number has effectiveRules specifying its arithmetic method and actual master-retention policy. Calculation rule IDs remain unchanged. No new calculation options, schools or SVG rendering are added here.

[Synthetic request](examples/profile-request.json) · [Full engine result](examples/profile-response.json) · [Filtered engine result](examples/profile-filtered-response.json)

Validation covers frozen 1.0.0 contracts (only engineVersion may differ), Vietnamese/Unicode/date/option matrices, multiple names, boundary sizes, known reconstruction paths, future-field canaries, a closed JSON schema and actual native/installed package consumers. Platform claims must match the environments actually tested.
