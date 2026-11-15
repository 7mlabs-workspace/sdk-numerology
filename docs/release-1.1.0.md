# Numerology SDK 1.1.0

[Tiếng Việt](release-1.1.0.vi.md) · **English**

[SDK home](../README.en.md) · [Formulas](features-and-formulas.md) · [Rules](rules.md) · [Filtered payload](payload-filtering.md)

Release date: 15 November 2026. MIT · Copyright (c) 2026 7mlabs.

## Release scope

Version 1.1.0 adds opt-in filtered output through the shared Rust core and Node/TypeScript, Python, .NET, C and CLI entry points. Existing six operations, Pythagorean arithmetic and Vietnamese normalization remain unchanged.

## 1.0.0 → 1.1.0

| Area | 1.0.0 | 1.1.0 |
| --- | --- | --- |
| Full APIs | Schema 1.0/1.1 with applicable source data | Same shape, arithmetic and errors; engineVersion becomes 1.1.0 |
| Filtered APIs | Unavailable | New calculateFiltered equivalents; separate filtered-1.0 schema |
| Raw totals and steps | Full output | Retained in full output and eligible filtered indicators |
| Master / reduced values | reducedValue through extensions or names | Existing behavior retained; every filtered Number includes reducedValue |
| Name text, letters and birth sources | Applicable full outputs | Full output unchanged; filtered output omits identified source groups |
| Attitude/Rational Thought traces | Available when calculated | Full output unchanged; filtered output retains results, withholds root/steps and gives a reason |
| Name/group IDs and warnings | Caller IDs | Filtered response-local labels and rebuilt structured warnings |
| Rule metadata | Versioned rules/options/extensions | Filtered calculationRules and per-Number effectiveRules |
| Formulas and Vietnamese names | 7mlabs Pythagorean 1.0 conventions | Unchanged |

## Upgrade and migration

Existing calculate calls continue to work. Upgrading the package **does not automatically filter existing calls**. Switch selected flows to calculateFiltered/calculate_filtered/CalculateFiltered and avoid logging original requests/full outputs. FilteredResult is a separate type and schema; it is not a full-output replacement object.

With masters enabled, a raw sum of 38 returns root 38, steps [38,11], value 11 and reducedValue 2. Disabling masters gives steps [38,11,2], value 2 and isMaster false. Attitude and Rational Thought retain their results but omit root/steps in filtered output because subtraction can expose birth-date components. They explicitly report traceStatus withheld and traceWithheldReason birthDateInference.

[Field policy and all entry points](payload-filtering.md) · [Complete formula catalog](features-and-formulas.md) · [Exact conventions](rules.md) · [Formula evidence](formula-verification.md)

## Distribution and verification

Publish the same 1.1.0 version to npm @7mlabs/numerology, PyPI sevenmlabs-numerology and NuGet SevenMLabs.Numerology. Verified native targets remain macOS ARM64/Linux x64; Python has two wheels. No other platform support is implied.

Validation includes frozen old contracts, 417 direct comparisons with the 1.0.0 binary, 66 Rust tests including a 2,700-configuration matrix, closed-schema and root/step checks, and actual installed-package consumers on both platforms. The compatibility comparison only permits engineVersion to differ. Release manifests bind tested files to source commits and SHA-256 hashes.

Filtered output reduces disclosure and **is not anonymous**. Combined indicators may still support identification; retained cycle traces can reconstruct the reference date and explicitly disclose that property. The calculation rules and Vietnamese Đ/diacritic/Y handling are unchanged; no alternate provider formulas or literal Telex-keystroke calculation is introduced.
