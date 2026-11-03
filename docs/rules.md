# Pythagorean calculation profile 1.0

Rules identifier: `pythagorean-7mlabs-1.0`. This identifies the exact conventions below, rather than claiming a universal standard shared by every practitioner. Outputs implement these arithmetic conventions; they do not establish the validity of personal predictions.

## Input and names

Dates are explicit Gregorian `YYYY-MM-DD`, years 0001–9999, including calendar validation. Cycles and a complete profile require `referenceDate`. There is no implicit computer date, timezone, random state or network service. Name components are explicit: `given`, optional `middle`, optional `family`. This avoids assuming Vietnamese or Western name order. Components may contain multiple words.

Names undergo Unicode NFD decomposition, removal of combining marks, Vietnamese Đ/đ to D, then ASCII uppercase conversion. Whitespace, hyphens and straight/curly apostrophes separate words. Other characters are rejected: unsupported scripts must be explicitly transliterated by the caller, never silently discarded. A name component is limited to 1024 UTF-8 bytes; given name must contain a supported letter. Requests are limited to 16 KiB. JSON duplicate/unknown fields and wrong types are rejected. Missing family and middle names are supported.

The letter table is A/J/S=1, B/K/T=2, C/L/U=3, D/M/V=4, E/N/W=5, F/O/X=6, G/P/Y=7, H/Q/Z=8, I/R=9. AEIOU are vowels. `yRule=vowel` and `consonant` apply an explicit choice to every Y. Default `contextual` treats Y as a vowel when it has no adjacent AEIOU in its word; otherwise as a consonant, separately for each occurrence. This is a spelling heuristic, not phonetic analysis; responses containing contextual Y include a warning. W remains a consonant. No empirical accuracy claim is made for this name convention.

## Reduction and name numbers

Reduction sums decimal digits until a single digit or, when `keepMasterNumbers=true` (default), 11/22/33. Every number exposes its unreduced root, reduction steps, result, master flag, and a karmic-debt value when 13/14/16/19 appears on that actual reduction path. Zero is valid for an absent category, such as no vowels or no family name; it does not signify a measured personality trait.

Expression is the sum of all name letters; Soul Urge is vowels; Personality is consonants. Balance uses the first letter of every word across the supplied components. Given and family name numbers use their respective component. Frequency counts the assigned values 1–9, missing numbers have zero frequency, Hidden Passion includes every most-frequent value, and Subconscious Self is 9 minus the count of missing values. Cornerstone and Capstone are the first and last given-name letters after normalization.

## Birth and derived numbers

Life Path reduces month, day and year separately with the configured master policy, then reduces their sum using that policy. Birthday reduces the day. Attitude reduces day + month without masters. Birth Grid counts nonzero digits in zero-padded Gregorian YYYYMMDD. Maturity reduces Life Path + Expression. Bridge values are the absolute differences of the reported Life Path/Expression and Soul Urge/Personality values; they are raw differences, including when a master number is retained.

## Cycles

For Pinnacles and Challenges, reduce birth month/day/year to single digits M/D/Y. P1=reduce(M+D), P2=reduce(D+Y), P3=reduce(P1.value+P2.value), P4=reduce(M+Y), with the configured master policy for pinnacle results. The first phase covers integer ages 0 through 36 minus single-digit Life Path, inclusive. The following two phases cover nine integer ages each. The fourth has no upper bound (`endAge=null`). These are integer age bands, not inferred current phases or calendar transition dates.

Challenges are |M−D|, |D−Y|, the absolute difference of the first two, and |M−Y|; no masters. Calendar-year Personal Year reduces M+D+single-digit(reference year) to 1–9. Personal Month reduces Personal Year + reference month; Personal Day reduces Personal Month + reference day. Universal Year reduces the reference year; Universal Month adds month, Universal Day adds day, reducing each to 1–9. All these cycles use the explicit reference date and the January–December convention, not a birthday rollover. This intentionally makes the current Personal Year consistent with its reported arithmetic.

## Essence

Each name component concatenates its normalized letters. A letter occupies as many integer age positions as its value, then the next letter starts; repeat the component cyclically. The given/middle/family components produce three independent transits. Empty optional components contribute null letter and zero. Essence reduces the sum of the three active letter values with the configured master policy. Age zero begins with the first letter. Requests can retrieve 1–120 rows inside ages 0–120. `birthYearPlusAge` is an arithmetic year label, not a birthday or calendar interval. No 99-year lifetime assumption is made.

## Scope and sources

Operations: `name`, `birth`, `cycles`, `essence`, `profile`. A profile combines these and derived numbers. Chaldean, interpretation prose, compatibility scoring, psychological archetypes and phone/address/business-number systems are not implemented or silently approximated. Further systems can be added with distinct rule identifiers without changing the meaning of this one.

Background practitioner references, used to identify conventions and disagreements rather than to certify empirical claims:

- Hans Decoz, [calculate your own reading](https://www.worldnumerology.com/do-your-own-reading/).
- Hans Decoz, [Y as vowel or consonant](https://www.worldnumerology.com/numerology-articles/numerology-Y-vowel-consonant.html).
- [Life Path component example](https://www.mynumerologycharts.com/life-path-calculator.html): 25 November 1975, components 7 + 11 + 22 = 40 → 4; reproduced as an arithmetic fixture.
- [Pinnacle conventions](https://tarot.ac/numerology/cycles/pinnacles).

The executable specification is this document plus named fixed arithmetic fixtures and invariant tests. No wording or copyrighted interpretation library is incorporated from these references.
