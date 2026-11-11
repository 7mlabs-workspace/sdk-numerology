<a href="../README.en.md"><img src="assets/readme/logo-7mlabs.png" width="180" alt="7mlabs full logo"></a>

# Pythagorean calculation profile 1.0

[Tiếng Việt](rules.vi.md) · **English**

[SDK home](../README.en.md) · [Features & formulas](features-and-formulas.md) · **Calculation rules** · [Formula evidence](formula-verification.md)

---

<details open>
<summary><strong>On this page</strong></summary>

- [Input and names](#doc-section-1)
- [Reduction and name numbers](#doc-section-2)
- [Birth and derived numbers](#doc-section-3)
- [Cycles](#doc-section-4)
- [Essence](#doc-section-5)
- [Scope and sources](#doc-section-6)
- [Opt-in extensions (payload schema 1.1)](#doc-section-7)
- [Multiple names and explicit combinations](#doc-section-8)
- [Opt-in first/last-letter name challenges](#doc-section-9)

</details>

Rules identifier: `pythagorean-7mlabs-1.0`. This identifies the exact conventions below, rather than claiming a universal standard shared by every practitioner. Outputs implement these arithmetic conventions; they do not establish the validity of personal predictions.

<a id="doc-section-1"></a>

## Input and names

Dates are explicit Gregorian `YYYY-MM-DD`, years 0001–9999, including calendar validation. Cycles and a complete profile require `referenceDate`. There is no implicit computer date, timezone, random state or network service. Name components are explicit: `given`, optional `middle`, optional `family`. This avoids assuming Vietnamese or Western name order. Components may contain multiple words.

Names undergo Unicode NFD decomposition, removal of combining marks, Vietnamese Đ/đ to D, then ASCII uppercase conversion. Whitespace, hyphens and straight/curly apostrophes separate words. Other characters are rejected: unsupported scripts must be explicitly transliterated by the caller, never silently discarded. A name component is limited to 1024 UTF-8 bytes; given name must contain a supported letter. Requests are limited to 16 KiB. JSON duplicate/unknown fields and wrong types are rejected. Missing family and middle names are supported.

The letter table is A/J/S=1, B/K/T=2, C/L/U=3, D/M/V=4, E/N/W=5, F/O/X=6, G/P/Y=7, H/Q/Z=8, I/R=9. AEIOU are vowels. `yRule=vowel` and `consonant` apply an explicit choice to every Y. Default `contextual` treats Y as a vowel when it has no adjacent AEIOU in its word; otherwise as a consonant, separately for each occurrence. This is a spelling heuristic, not phonetic analysis; responses containing contextual Y include a warning. W remains a consonant. No empirical accuracy claim is made for this name convention.

<a id="doc-section-2"></a>

## Reduction and name numbers

Reduction sums decimal digits until a single digit or, when `keepMasterNumbers=true` (default), 11/22/33. Every number exposes its unreduced root, reduction steps, result, master flag, and a karmic-debt value when 13/14/16/19 appears on that actual reduction path. Zero is valid for an absent category, such as no vowels or no family name; it does not signify a measured personality trait.

Expression is the sum of all name letters; Soul Urge is vowels; Personality is consonants. Balance uses the first letter of every word across the supplied components. Given and family name numbers use their respective component. Frequency counts the assigned values 1–9, missing numbers have zero frequency, Hidden Passion includes every most-frequent value, and Subconscious Self is 9 minus the count of missing values. Cornerstone and Capstone are the first and last given-name letters after normalization.

<a id="doc-section-3"></a>

## Birth and derived numbers

Life Path reduces month, day and year separately with the configured master policy, then reduces their sum using that policy. Birthday reduces the day. Attitude reduces day + month without masters. Birth Grid counts nonzero digits in zero-padded Gregorian YYYYMMDD. Maturity reduces Life Path + Expression. Bridge values are the absolute differences of the reported Life Path/Expression and Soul Urge/Personality values; they are raw differences, including when a master number is retained.

<a id="doc-section-4"></a>

## Cycles

For Pinnacles and Challenges, reduce birth month/day/year to single digits M/D/Y. P1=reduce(M+D), P2=reduce(D+Y), P3=reduce(P1.value+P2.value), P4=reduce(M+Y), with the configured master policy for pinnacle results. The first phase covers integer ages 0 through 36 minus single-digit Life Path, inclusive. The following two phases cover nine integer ages each. The fourth has no upper bound (`endAge=null`). These are integer age bands, not inferred current phases or calendar transition dates.

Challenges are |M−D|, |D−Y|, the absolute difference of the first two, and |M−Y|; no masters. Calendar-year Personal Year reduces M+D+single-digit(reference year) to 1–9. Personal Month reduces Personal Year + reference month; Personal Day reduces Personal Month + reference day. Universal Year reduces the reference year; Universal Month adds month, Universal Day adds day, reducing each to 1–9.

All these cycles use the explicit reference date and the January–December convention, not a birthday rollover. This intentionally makes the current Personal Year consistent with its reported arithmetic.

<a id="doc-section-5"></a>

## Essence

Each name component concatenates its normalized letters. A letter occupies as many integer age positions as its value, then the next letter starts; repeat the component cyclically. The given/middle/family components produce three independent transits. Empty optional components contribute null letter and zero. Essence reduces the sum of the three active letter values with the configured master policy. Age zero begins with the first letter.

Requests can retrieve 1–120 rows inside ages 0–120. `birthYearPlusAge` is an arithmetic year label, not a birthday or calendar interval. No 99-year lifetime assumption is made.

<a id="doc-section-6"></a>

## Scope and sources

Operations: `name`, `birth`, `cycles`, `essence`, `profile`, plus the `names` operation documented below. A profile combines the four base calculation scopes and derived numbers. See the [feature/formula catalog](features-and-formulas.md) for an indicator-to-payload map. Chaldean, interpretation prose, compatibility scoring, psychological archetypes and phone/address/business-number systems are not implemented or silently approximated. Further systems can be added with distinct rule identifiers without changing the meaning of this one.

Background practitioner references, used to identify conventions and disagreements rather than to certify empirical claims:

- Hans Decoz, [calculate your own reading](https://www.worldnumerology.com/do-your-own-reading/).
- Hans Decoz, [Y as vowel or consonant](https://www.worldnumerology.com/numerology-articles/numerology-Y-vowel-consonant.html).
- [Life Path component example](https://www.mynumerologycharts.com/life-path-calculator.html): 25 November 1975, components 7 + 11 + 22 = 40 → 4; reproduced as an arithmetic fixture.
- [Pinnacle conventions](https://tarot.ac/numerology/cycles/pinnacles).

The executable specification is this document plus named fixed arithmetic fixtures and invariant tests. No wording or copyrighted interpretation library is incorporated from these references.

<a id="doc-section-7"></a>

## Opt-in extensions (payload schema 1.1)

The base rules above, existing `Number` fields and schema 1.0 output are unchanged. Omit `extensions` (or pass JSON null) for the previous payload. An extensions object selects schema 1.1 and reports effective switches plus `pythagorean-7mlabs-extensions-1.0` in result metadata. No extension is enabled by the current date or by input-name content. The combined request remains bounded to 16 KiB. Unknown or duplicate fields and wrong types remain errors.

Both switches default to true: `reducedValues` controls number enrichment; `indicators` controls the new `data.extended` indicators. A `currentName` uses the same explicit `given`/`middle`/`family` structure, limits, normalization and Y configuration as the birth name. It is optional and is rejected when indicators are disabled or the operation is not `name` or `profile`. Normalization errors identify the current-name field. A contextual Y in a current name produces its own warning. The SDK does not split comma-separated aliases or choose a preferred name automatically.

| Operation | Indicators in data.extended |
|---|---|
| name | Birth-name grid axes; current-name numbers and explicitly configured name Challenges when supplied |
| birth | Birth-grid axes |
| cycles | Three Period Cycles |
| essence | No additional indicator section; reduced number enrichment is available |
| profile | All applicable additions above, Rational Thought, combined grid and an explicitly requested name set |

### Reduced values and display

`reducedValue` is the selected `value` reduced to one digit with no master retention; zero stays zero. It never replaces `value` or appends to `steps`. For example, a retained master is `{root:33, value:33, steps:[33], isMaster:true, karmicDebt:null, reducedValue:6}`. If master retention is disabled, `value=6`, `steps=[33,6]` and `isMaster=false`; the extension does not resurrect a retained master. Compound 10 is not a master, regardless of how a UI displays 10/1.

Existing derived Bridges continue using their documented reported values, not display-only reduced values. Name reduction remains the configured whole-name sum; this addition does not switch to per-component reduction.

### Rational Thought

For a profile, sum all letters in the normalized birth `name.given`, add the original day of the month and reduce with no master retention. The output labels `nameSource=birthName.given`. Example: THOMAS = 22, day 3 gives 25 → 7. Current names and family/middle parts do not enter this calculation. This follows the Rational Thought arithmetic described by [Hans Decoz](https://www.worldnumerology.com/do-your-own-reading/); it does not claim to reproduce every website's similarly named indicator. A single comparison case is insufficient to infer an alternative first-name convention.

### Three Period Cycles

The values are the birth month, day and year respectively, reduced with the existing `keepMasterNumbers` setting. The first integer age band is 0 through `36 - singleDigit(LifePath)`, inclusive. The second starts at the next integer age and includes 27 consecutive integer ages. The third starts at the next integer age and has no end (`endAge=null`). The `source` field distinguishes birthMonth/birthDay/birthYear.

These are integer age bands; they are not calendar intervals or an assertion about the current phase. They are distinct from four Pinnacles. The value sequence and 27-year middle period are described in the [Period Cycle reference](https://www.worldnumerology.com/do-your-own-reading/); the inclusive integer convention is explicitly fixed here.

### Grids and axes

Each of eight triples is reported by its digits and a neutral status: 1–2–3, 4–5–6, 7–8–9, 1–4–7, 2–5–8, 3–6–9, 1–5–9, 3–5–7. All three counts positive means `present`, all zero means `absent`, otherwise `partial`. No psychological label or interpretation prose is implied. Birth-name axes use the original full birth-name frequency; birth axes use Gregorian birth digits.

For profiles the combined grid is an exact per-digit sum of the birth grid and the selected name frequency. If `currentName` is provided, its frequency is used; otherwise the full birth name is used. The result explicitly labels `nameSource`, keeps `nameFrequency`, and reports combined frequency and axes. This is a transparent arithmetic combination, not a learned scoring model.

### Current-name numbers and remaining scope

Current-name numbers reuse the existing name calculation on only the caller-supplied current name. They are separate from birth-name data; they do not enter the base Maturity, Bridges, Essence or birth-name Rational Thought. The caller chooses all name parts and may supply a given-only current name. The [Minor number reference](https://www.worldnumerology.com/do-your-own-reading/) uses a current full name; a given-only input is explicitly the caller's chosen scope, not a promise of equivalence to that reference or a proprietary Approach system.

The extensions implement the two explicitly configured first/last-letter name Challenges documented below. They do not infer Expression Challenge or the comparison website's Năng lực trưởng thành convention, fixed global karmic-debt lists, personality/career percentages, compatibility scores, Chaldean or interpretation prose. Unimplemented additions require explicit formula/scope decisions. A current-name calculation matching one website example does not certify its full algorithm.

<a id="doc-section-8"></a>

## Multiple names and explicit combinations

Rules identifier: `pythagorean-7mlabs-names-1.0`. This is a transparent caller-selected composite calculation, not a claim that Pythagorean practitioners merge every nickname. The `name` operation and all preexisting requests/results remain unchanged.

### Standalone operation

`operation=names` accepts `names: [{id, name}]`, optional `combinations: [{id, sources}]`, optional `birthDate`, and the existing `system` and `options`. Every input uses the same `given`/`middle`/`family` structure, normalization, Y rule, master-number policy and name-number core as the single-name operation. IDs are case-sensitive, unique across names and combinations, 1–64 ASCII characters matching `[A-Za-z0-9][A-Za-z0-9_.-]*`. The order of each output list is the request order. A standalone `birth` ID is an ordinary caller ID, not a verified identity label.

The response is schema 1.1 with the base rules version, `namesRulesVersion`, and `data.names`. That object contains its own rulesVersion, independent `names`, and `combinations`. Single results include an ID, provenance (`input`), full number results and, if birthDate is provided, Rational Thought using only that entry's given component. They never replace numbers in another entry. With no birthDate, Rational Thought is omitted; an invalid provided date is rejected.

Number results include reducedValue, without changing their normal value or trace. The JSON request size limit remains 16 KiB.

### Explicit source selection and reduction

Each group source has `nameId` and optional `part`: `full` (default), `given`, `middle` or `family`. A full source expands nonempty components in the deterministic given/middle/family order; callers needing a different order explicitly list disjoint parts. Normalized components and sources are joined with spaces, preserving word boundaries for Y classification. Each result echoes the ordered source IDs, effective parts and selected text. Combinations cannot reference other combinations, so there are no recursive groups or generated permutations.

The composite is evaluated by the same name core on the joined text as one synthetic given component, with middle/family empty. It reports `method=concatenateSelectedParts`, `reduction=wholeNameSum`, normalized text, raw totals and reduction traces. It is not calculated by adding previously reduced numbers: for example, AIIII has total 37 → 1, and A has total 1; the combined original total is 38 → 11 under master retention, not 1+1=2. When master retention is disabled, the combined path is 38 → 11 → 2.

If birthDate is supplied, a group's Rational Thought adds the original day of month to the selected text's full letter sum. The result labels `nameSource=selectedText`. This is distinct from the birth-name Rational Thought of a profile and is not automatically substituted for it. Composite Cornerstone/Capstone and Balance describe that synthetic ordered scope, not a person's birth-name identity.

### Duplicate and size handling

All individual names are preserved, even if accent/case normalization makes their values identical. Groups reject repeated normalized selected text and repeated/overlapping component selections of the same normalized name, including through duplicate input IDs. This is an error, not silent deduplication. Disjoint given/family selections of one name are allowed. Empty selected optional components return EMPTY_NAME_PART. Missing references return UNKNOWN_NAME_ID. Invalid IDs, duplicate IDs and duplicate sources return structured field-specific errors.

Bounds are 1–32 total input names, 0–32 groups, 2–32 sources in each group, and 4096 UTF-8 bytes in each joined normalized composite. Each input name component still has the existing 1024 UTF-8 byte limit. No fuzzy spelling matching, automatic aliases, weighting or semantic interpretation is introduced.

### Profile orchestration and compatibility

A profile may specify `extensions.nameSet: {names, combinations}`. The profile's existing normalized birth name is injected as ID `birth`, source `profile.birthName`; names contains at most 31 additional caller inputs. Omitted arrays default to empty and a profile nameSet object alone returns just the birth entry. The ID birth is reserved in this mode. Output is in `data.extended.nameSet` and the names rules version is recorded in extension metadata. This requires indicators=true and operation profile; other uses return INVALID_EXTENSION instead of being ignored.

Name-set results honor the profile's reducedValues switch. Existing profile fields—including the original birth-name Rational Thought, Maturity, Bridges, Essence, currentName and combined-grid name choice—are unchanged. A group is not automatically used as the profile's currentName or to recalculate its birth-name values. Callers may compare any explicitly selected scopes using the separate results.

Rust callers have separate `NamesRequest` / `calculate_names` and `NameSet` / `calculate_with_name_set` APIs. Existing public Request, Extensions and Number struct layouts and their old functions are preserved. All language bindings use the same JSON implementation; this feature has no network dependency.

For TypeScript source compatibility, the existing Request and Operation aliases retain their pre-multiple-name domains. NamesRequest describes the new standalone input; AnyRequest and SupportedOperation cover both old operations and names. Existing Result generics remain valid for old Operation constraints; NamesResult describes the new response.

<a id="doc-section-9"></a>

## Opt-in first/last-letter name challenges

`extensions.nameChallenges` is absent/null by default. A nonnull object requires explicit `nameOrder` (`givenMiddleFamily` or `familyMiddleGiven`), `operation=name|profile` and `indicators=true`. It is applied separately to `data.name.normalizedName` and an optional normalized `currentName`, never to birth-derived numbers, existing name sums, name sets or combined grids. Empty middle/family parts are skipped; token boundaries and the caller's existing Y policy are preserved.

The namespace `data.extended.nameChallenges` contains `birthName` and optional `currentName`. Each includes `rulesVersion=pythagorean-7mlabs-name-challenges-1.0`, `orderedText`, `nameOrder`, `yRule`, methods/zero policies, and the `soulUrge`/`personality` calculation. Endpoint letters carry their 1–9 Pythagorean values. `difference` is their absolute difference. Soul Urge retains zero (Blair Andrews / Numerologist); Personality maps zero to nine (Roy Kirkland). `keepMasterNumbers` and `reducedValues` do not affect these bounded differences. These are not `CalculatedNumber` objects and do not receive karmic-debt/master labels.

At least one matching letter is required: a single vowel/consonant is both endpoints, producing difference zero; no matching letters returns `status=unavailable`, `reason=noMatchingLetters` and null `first`, `last`, `difference`, `value`. An available result has `status=available`. The top-level extension metadata adds `nameChallengesRulesVersion` only on opt-in. Existing Rust structs and previously accepted JSON requests stay unchanged. A standalone Rust `calculate_name_challenges(&Name,&Options,&NameChallengeOptions)` supports any explicitly supplied name/alias. The new options and order types are separate; no defaults guess cultural name order.

Formula sources and the two unresolved website-specific mappings are documented in [formula verification](formula-verification.md).
