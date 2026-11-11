<a href="../README.en.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs full logo"></a>

# Supported features and calculation formulas

[Tiếng Việt](features-and-formulas.vi.md) · **English**

[SDK home](../README.en.md) · **Features & formulas** · [Calculation rules](rules.md) · [Formula evidence](formula-verification.md)

---

<details open>
<summary><strong>On this page</strong></summary>

- [System and conventions](#doc-section-1)
- [Choose the calculation scope](#doc-section-2)
- [One name: sums, frequencies and endpoint letters](#doc-section-3)
- [Birth date and derived values](#doc-section-4)
- [Calendar cycles, Pinnacles and birth-date Challenges](#doc-section-5)
- [Transits and Essence](#doc-section-6)
- [Optional additions](#doc-section-7)
- [Reading a number and reproducing a payload](#doc-section-8)
- [Boundaries](#doc-section-9)
- [Implementation and evidence](#doc-section-10)

</details>

This catalog describes SDK 1.0.0, including multi-name calculations and optional schema 1.1 additions. Match packages to the tested commit and hashes in the release manifest. See [the README](../README.en.md) for build/use instructions and [the exact rules](rules.md) for validation and edge cases.

<a id="doc-section-1"></a>

## System and conventions

Only `system=pythagorean` is implemented. Omission selects that system; another value returns `UNSUPPORTED_SYSTEM`. The response identifies the actual system, `rulesVersion`, engine version, schema version and effective options. Extensions have separate rule identifiers. These identifiers specify 7mlabs' conventions, not agreement among every practitioner or calculator.

Definitions used below:

- `R(n)`: sum decimal digits repeatedly, stopping at a single digit or at 11, 22 or 33 when `keepMasterNumbers=true`. With that option false it is the same as `D(n)`.
- `D(n)`: sum decimal digits repeatedly to one digit, retaining zero and never stopping at a master number.
- `V(text)`: sum the Pythagorean letter values of normalized text. Spaces separate words and have no numerical value.
- `LP`, `EX`, `SU`, `PE`: reported Life Path, Expression, Soul Urge and Personality values, including retained masters.
- `M`, `B`, `Y`: single-digit birth month, day and year: `D(birthMonth)`, `D(birthDay)`, `D(birthYear)`.
- `E = 36 - D(LP)`: inclusive end age of the first Pinnacle and first Period Cycle.

The letter table is:

| Value | Letters |
| --- | --- |
| 1 | A J S |
| 2 | B K T |
| 3 | C L U |
| 4 | D M V |
| 5 | E N W |
| 6 | F O X |
| 7 | G P Y |
| 8 | H Q Z |
| 9 | I R |

Names are normalized to ASCII uppercase with decomposable accents removed and Vietnamese Đ converted to D. Hyphens, apostrophes and whitespace separate words. Other scripts must be explicitly transliterated by the caller. AEIOU are vowels; W is a consonant. Y uses the caller's `vowel`/`consonant` choice or the documented default spelling heuristic: Y is a vowel only when neither adjacent letter in its word is AEIOU. This is not pronunciation detection.

<a id="doc-section-2"></a>

## Choose the calculation scope

These are JSON operation names, not separate sets of arithmetic implementations for each language. Node.js/TypeScript, Python, .NET and the JSON CLI use the same Rust core. Output paths below are relative to the unwrapped `result`; the raw API surrounds it with `{ok, result}` or `{ok, error}`.

| Operation | Required input | Output scope |
| --- | --- | --- |
| `name` | Structured `name` | `data.name`: one supplied name; an alias may be supplied as its own input. |
| `birth` | `birthDate` | `data.birth`: date-derived numbers and birth grid. |
| `cycles` | `birthDate`, `referenceDate` | `data.cycles`: six calendar cycles, four Pinnacles and four birth-date Challenges. |
| `essence` | `name`, `birthDate` | `data.essence`: requested age rows with three transits and Essence. |
| `profile` | `name`, `birthDate`, `referenceDate` | All four base scopes plus `data.derived`; optional additions described below. |
| `names` | Nonempty `names` list | `data.names`: each name independently and only caller-selected combinations; optional `birthDate` enables scoped Rational Thought. |

`given` is required; `middle` and `family` are optional and may contain multiple words. Dates are explicit Gregorian `YYYY-MM-DD`. Calendar calculations never take the date from the machine clock. `startAge` defaults to 0 and `years` to 100 for Essence/profile; rows must stay within ages 0–120. See [input rules and bounds](rules.md#input-and-names).

<a id="doc-section-3"></a>

## One name: sums, frequencies and endpoint letters

For a base name calculation, full text includes all given/middle/family components. Current-name and individual multi-name calculations reuse these formulas on their own supplied name. A selected combination uses its explicitly joined text as a synthetic given component; it does not become the profile's birth name.

| Feature | Formula / precise scope | Path under `data.name` |
| --- | --- | --- |
| Expression / Sứ mệnh | `R(V(fullName))` | `expression` |
| Soul Urge / Linh hồn | `R(sum of vowel values in fullName)` using the selected Y rule | `soulUrge` |
| Personality / Nhân cách | `R(sum of consonant values in fullName)` | `personality` |
| Balance / Cân bằng | `R(sum of the first-letter values of every word across all supplied components)` | `balance` |
| Given-name number | `R(V(given))`; includes every word in the given component | `givenName` |
| Family-name number | `R(V(family))`; empty family produces zero | `familyName` |
| Name grid | Counts of letter values 1–9 across the full name | `frequency` |
| Missing numbers / Bài học theo số thiếu | Every value 1–9 with zero name frequency | `missingNumbers` |
| Hidden Passion / Đam mê tiềm ẩn | Every value tied for the largest name frequency | `hiddenPassion` |
| Subconscious Self / Tiềm thức | `9 - count(missingNumbers)` | `subconsciousSelf` |
| Cornerstone | First normalized letter of `given` | `cornerstone` |
| Capstone | Last normalized letter of `given` | `capstone` |

An absent vowel/consonant category in the **sum-based** Soul Urge/Personality numbers has sum zero. Do not confuse that convention with unavailable endpoint-based Challenges below.

<a id="doc-section-4"></a>

## Birth date and derived values

| Feature | Formula / precise scope | Output path |
| --- | --- | --- |
| Life Path / Đường đời | `R(R(birthMonth) + R(birthDay) + R(birthYear))`; master policy applies to each component and the final sum | `data.birth.lifePath` |
| Birthday / Ngày sinh | `R(birthDay)` | `data.birth.birthDay` |
| Attitude / Thái độ | `D(birthMonth + birthDay)` | `data.birth.attitude` |
| Reduced date components | `R(birthMonth)`, `R(birthDay)`, `R(birthYear)` | `data.birth.components` |
| Birth grid | Count nonzero digits of zero-padded Gregorian `YYYYMMDD`; zeros are excluded | `data.birth.birthGrid` |
| Maturity / Trưởng thành | `R(LP + EX)` from the profile's birth name and birth date | `data.derived.maturity` |
| Life Path–Expression Bridge | `abs(LP - EX)` using the reported values | `data.derived.lifePathExpressionBridge` |
| Soul Urge–Personality Bridge | `abs(SU - PE)` using the reported values | `data.derived.soulPersonalityBridge` |

Bridges are raw differences, not `R` or `D` results. For example, reported `LP=9`, `EX=11` produces Bridge 2; this SDK does not silently replace 11 by its display root 2 and return 7. Maturity is a separate calculation. Neither Bridge is labeled as a verified equivalent of the comparison website's “Năng lực trưởng thành”.

<a id="doc-section-5"></a>

## Calendar cycles, Pinnacles and birth-date Challenges

Let `ry`, `rm`, `rd` be the original year, month and day of the explicitly supplied reference date.

| Calendar feature | Formula | Path under `data.cycles` |
| --- | --- | --- |
| Personal Year | `D(M + B + D(ry))` | `personalYear` |
| Personal Month | `D(PersonalYear + rm)` | `personalMonth` |
| Personal Day | `D(PersonalMonth + rd)` | `personalDay` |
| Universal Year | `D(ry)` | `universalYear` |
| Universal Month | `D(UniversalYear + rm)` | `universalMonth` |
| Universal Day | `D(UniversalMonth + rd)` | `universalDay` |

All six use single digits without retained masters. The Personal Year follows January–December, not a birthday rollover. Roots and intermediate steps follow the sums shown above, including the original reference month/day.

| Pinnacle | Formula | Inclusive integer ages |
| --- | --- | --- |
| 1 | `P1 = R(M + B)` | `0..E` |
| 2 | `P2 = R(B + Y)` | `E+1..E+9` |
| 3 | `P3 = R(P1.value + P2.value)` | `E+10..E+18` |
| 4 | `P4 = R(M + Y)` | `E+19` onward; `endAge=null` |

Pinnacles are in `data.cycles.pinnacles` with one-based `phase`. Their input date components are single digits even when a component in `data.birth.components` retains a master. The third Pinnacle uses the first two **reported Pinnacle values**, including retained masters.

The four birth-date Challenges in `data.cycles.challenges`, in phase order, are `C1=abs(M-B)`, `C2=abs(B-Y)`, `C3=abs(C1-C2)`, `C4=abs(M-Y)`. They allow zero and never retain masters. These four values are distinct from the two name-based Challenges below. Age bands describe integer ages, not exact transition dates or an inferred current phase.

<a id="doc-section-6"></a>

## Transits and Essence

For each given/middle/family component independently, remove word separators. Each letter occupies as many integer age positions as its 1–9 value; proceed to the next letter and repeat the sequence cyclically. At age `a`, the position is `a mod V(component)`. Age zero uses the first letter. An empty optional component contributes `{letter:null, value:0}`.

For each row, `Essence = R(givenTransit.value + middleTransit.value + familyTransit.value)`. Results are in `data.essence.years`, with active letters and values for each component. `birthYearPlusAge` is the arithmetic label `birthYear + age`, not a date interval or a timezone conversion.

<a id="doc-section-7"></a>

## Optional additions

Omitting `extensions` or passing null preserves schema 1.0. An object opts into schema 1.1. `reducedValues` and `indicators` default to true; they can be disabled independently. Number sums and old result fields retain their existing meanings.

| Addition | Formula / behavior | Output / activation |
| --- | --- | --- |
| Master display root | `reducedValue = D(value)`; does not change `value`, `steps` or `isMaster` | Added to number objects when `reducedValues=true`; standalone `names` always includes it |
| Rational Thought / Tư duy | `D(V(birthName.given) + original birthDay)` | `data.extended.rationalThought` on `profile`; source is `birthName.given` |
| Three Period Cycles | Values `R(birthMonth)`, `R(birthDay)`, `R(birthYear)`; age bands `0..E`, `E+1..E+27`, `E+28` onward | `data.extended.periodCycles` on `cycles`/`profile` |
| Grid-axis coverage | Eight digit triples: 123, 456, 789, 147, 258, 369, 159, 357. All counts positive → `present`; all zero → `absent`; otherwise `partial` | `birthGridAxes` on `birth`/`profile`; `birthNameGridAxes` on `name`/`profile` |
| Current-name numbers | Same name formulas, calculated on the separate supplied current name | `extensions.currentName`; output `data.extended.currentName` on `name`/`profile` |
| Combined grid | Per-digit sum of birth-grid frequency and selected name frequency; uses `currentName` when supplied, otherwise birth name | `data.extended.combinedGrid` on `profile`; labels `nameSource` and includes axis coverage |
| Many names | Same name formulas independently for each caller ID | Standalone `names`, or `extensions.nameSet` on `profile` |
| Selected name combinations | Join explicitly selected full/given/middle/family texts in caller order; apply the same name formulas to the joined original letters | `data.names.combinations` or `data.extended.nameSet.combinations` |

Grid axes are digit-coverage calculations, without psychological labels or interpretation. Current names and groups never automatically replace the birth name in Maturity, Bridges, Essence or birth-name Rational Thought.

### Many names and combinations

Names and groups keep caller IDs and source order. The SDK does not generate nickname combinations automatically. A profile reserves `birth` for its birth name; standalone `names` has no injected birth identity. A `full` source uses given/middle/family order; explicit disjoint parts allow another order.

Combinations sum **original letter values** before reduction, not already reduced results. Example: AIIII totals 37 → 1 and A totals 1; their joined original total is 38 → 11 with master retention, not 1+1=2. The result records `method=concatenateSelectedParts`, `reduction=wholeNameSum` and each selected source/text.

When `birthDate` is supplied to `names`, each entry gets Rational Thought from its own given component and birth day. Groups use `D(V(selectedText) + original birthDay)` and label the scope `selectedText`. A profile name set gets the profile birth date but does not change the original profile Rational Thought. See [collection limits, duplicates and source selection](rules.md#multiple-names-and-explicit-combinations).

### Two name-based Challenges

Only `name`/`profile` support `extensions.nameChallenges`; it requires `indicators=true` and explicit `nameOrder=givenMiddleFamily|familyMiddleGiven`. It calculates birth name and optional current name separately, without automatically extending `names`/`nameSet` groups. Rust also exposes `calculate_name_challenges` for an explicitly supplied name.

| Challenge | Formula / convention | Path under `data.extended.nameChallenges.birthName` |
| --- | --- | --- |
| Soul Urge Challenge / Thử thách Linh hồn | Absolute difference of first/last vowel values in the ordered name; retain zero, following the documented Numerologist convention | `soulUrge` |
| Personality Challenge / Thử thách Nhân cách | Absolute difference of first/last consonant values; map raw zero to 9, following Roy Kirkland's convention | `personality` |

The same configured Y classification is used here. Results include ordered text, name order, Y rule, method, zero policy, endpoint letters/values, raw `difference` and final `value`. One matching letter serves as both endpoints and gives difference zero. No matching letters yields `status=unavailable` and null endpoints/difference/value. These endpoint results are not master/karmic-debt number objects. [Evidence, worked examples and unresolved methods](formula-verification.md) distinguish sourced conventions from unverified website mappings.

<a id="doc-section-8"></a>

## Reading a number and reproducing a payload

A normal calculated number includes `root`, `steps`, `value`, `isMaster` and `karmicDebt`. Debt is recorded only when 13/14/16/19 occurs on that actual reduction path. It is not a fixed global list or a union across all dates and name scopes. Optional `reducedValue` is for display; for example, `value=11`, `isMaster=true`, `reducedValue=2` may be displayed as 11/2.

The synthetic [example request](examples/profile-request.json) and its [exact CLI response](examples/profile-response.json) demonstrate a full profile, separate current name, one selected name group and explicit first/last-letter order. They use no real person's report. Selected results are Life Path 6, Expression 5 and Maturity 11, with a display root of 2 for Maturity. The raw response includes all reduction steps and method metadata; callers should not infer formulas from translated labels alone.

For existing applications, leave the request unchanged to preserve its current payload. New features are explicitly requested; no previously returned value is silently replaced by a new name, group or display root.

<a id="doc-section-9"></a>

## Boundaries

- Implemented system: Pythagorean only. Chaldean, Vedic and other systems have no calculation implementation in this source.
- The website-specific Expression Challenge and “Năng lực trưởng thành” formulas remain unresolved and are not added. Maturity is already supported under its documented formula.
- No interpretation library, personality/career percentages, compatibility score, automatic name recommendations, or phone/address/business-number system is included.
- Current native verification is macOS ARM64. Other targets require their own build and tests; source-level language bindings do not imply every native binary is available.
- The SDK is offline and deterministic. It produces arithmetic results and calculation metadata; applications choose presentation and interpretation.

<a id="doc-section-10"></a>

## Implementation and evidence

The implementation behind the tables is [the base core](../crates/numerology-core/src/lib.rs), [extensions](../crates/numerology-core/src/extensions.rs), [name collections](../crates/numerology-core/src/names.rs) and [name challenges](../crates/numerology-core/src/name_challenges.rs). [Exact rule definitions](rules.md) and [formula evidence](formula-verification.md) record conventions and disagreements. Independent arithmetic examples and compatibility fixtures are under [core tests](../crates/numerology-core/tests) and [contract fixtures](../tests/fixtures).
