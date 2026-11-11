//! Deterministic Pythagorean calculations. No clock, network or mutable global state.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};
mod extensions;
pub use extensions::{calculate_with_extensions, Extensions};
mod name_challenges;
pub use name_challenges::{calculate_name_challenges, NameChallengeOptions, NameOrder};
mod names;
pub use names::{
    calculate_names, calculate_with_name_set, NameCombination, NameInput, NamePart, NameSet,
    NameSource, NamesRequest,
};
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RULES: &str = "pythagorean-7mlabs-1.0";
pub const MAX_REQUEST_BYTES: usize = 16384;
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: &'static str,
    pub field: String,
    pub message: String,
}
fn err(code: &'static str, field: &str, message: &str) -> Error {
    Error {
        code,
        field: field.into(),
        message: message.into(),
    }
}
type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Name {
    pub given: String,
    #[serde(default)]
    pub middle: String,
    #[serde(default)]
    pub family: String,
}
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum YRule {
    Vowel,
    Consonant,
    #[default]
    Contextual,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Options {
    #[serde(default = "yes")]
    pub keep_master_numbers: bool,
    #[serde(default)]
    pub y_rule: YRule,
}
fn yes() -> bool {
    true
}
impl Default for Options {
    fn default() -> Self {
        Self {
            keep_master_numbers: true,
            y_rule: YRule::Contextual,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Request {
    pub operation: String,
    #[serde(default = "system")]
    pub system: String,
    pub name: Option<Name>,
    pub birth_date: Option<String>,
    pub reference_date: Option<String>,
    #[serde(default)]
    pub options: Options,
    #[serde(default)]
    pub start_age: u16,
    #[serde(default = "hundred")]
    pub years: u16,
}
fn system() -> String {
    "pythagorean".into()
}
fn hundred() -> u16 {
    100
}
// Keep Request's public Rust layout unchanged. The JSON wire can opt into extensions
// without adding required fields to existing callers' Request struct literals.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WireRequest {
    operation: String,
    #[serde(default = "system")]
    system: String,
    name: Option<Name>,
    birth_date: Option<String>,
    reference_date: Option<String>,
    #[serde(default)]
    options: Options,
    #[serde(default)]
    start_age: u16,
    #[serde(default = "hundred")]
    years: u16,
    extensions: Option<WireExtensions>,
    #[serde(default, deserialize_with = "present_vec")]
    names: Option<Vec<NameInput>>,
    #[serde(default, deserialize_with = "present_vec")]
    combinations: Option<Vec<NameCombination>>,
}
fn present_vec<'de, D, T>(deserializer: D) -> std::result::Result<Option<Vec<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Vec::<T>::deserialize(deserializer).map(Some)
}
// Preserve the already-public Extensions struct as well as Request.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WireExtensions {
    #[serde(default = "yes")]
    reduced_values: bool,
    #[serde(default = "yes")]
    indicators: bool,
    current_name: Option<Name>,
    name_set: Option<NameSet>,
    name_challenges: Option<NameChallengeOptions>,
}
impl WireRequest {
    fn calculate(self) -> Result<Value> {
        if self.operation == "names" {
            if self.name.is_some()
                || self.reference_date.is_some()
                || self.extensions.is_some()
                || self.start_age != 0
                || self.years != 100
            {
                return Err(err(
                    "INVALID_REQUEST",
                    "request",
                    "names accepts names, combinations, optional birthDate, system and options",
                ));
            }
            return calculate_names(&NamesRequest {
                names: self.names.ok_or_else(|| {
                    err(
                        "MISSING_FIELD",
                        "names",
                        "Provide a nonempty list of named inputs",
                    )
                })?,
                combinations: self.combinations.unwrap_or_default(),
                birth_date: self.birth_date,
                system: self.system,
                options: self.options,
            });
        }
        if self.names.is_some() || self.combinations.is_some() {
            return Err(err(
                "INVALID_REQUEST",
                "request",
                "Top-level names and combinations require operation names; profiles use extensions.nameSet",
            ));
        }
        let request = Request {
            operation: self.operation,
            system: self.system,
            name: self.name,
            birth_date: self.birth_date,
            reference_date: self.reference_date,
            options: self.options,
            start_age: self.start_age,
            years: self.years,
        };
        match self.extensions {
            Some(wire) => {
                let extensions = Extensions {
                    reduced_values: wire.reduced_values,
                    indicators: wire.indicators,
                    current_name: wire.current_name,
                };
                let mut output = match wire.name_set {
                    Some(set) => calculate_with_name_set(&request, &extensions, &set),
                    None => calculate_with_extensions(&request, &extensions),
                }?;
                if let Some(settings) = wire.name_challenges {
                    name_challenges::add_to_result(&mut output, &settings, extensions.indicators)?;
                }
                Ok(output)
            }
            None => calculate(&request),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Number {
    pub root: u32,
    pub value: u32,
    pub steps: Vec<u32>,
    pub is_master: bool,
    pub karmic_debt: Option<u32>,
}
pub fn reduce(root: u32, keep: bool) -> Number {
    let mut n = root;
    let mut steps = vec![n];
    let mut karmic = None;
    loop {
        if [13, 14, 16, 19].contains(&n) {
            karmic = Some(n)
        }
        if n < 10 || (keep && [11, 22, 33].contains(&n)) {
            break;
        }
        let mut t = n;
        let mut sum = 0;
        while t > 0 {
            sum += t % 10;
            t /= 10
        }
        n = sum;
        steps.push(n);
    }
    Number {
        root,
        value: n,
        steps,
        is_master: [11, 22, 33].contains(&n),
        karmic_debt: karmic,
    }
}
#[derive(Clone, Copy, Debug)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}
impl Date {
    fn parse(s: &str, field: &str) -> Result<Self> {
        let b = s.as_bytes();
        if b.len() != 10
            || b[4] != b'-'
            || b[7] != b'-'
            || b.iter()
                .enumerate()
                .any(|(i, c)| i != 4 && i != 7 && !c.is_ascii_digit())
        {
            return Err(err(
                "INVALID_DATE",
                field,
                "Use a Gregorian date YYYY-MM-DD, years 0001..9999",
            ));
        }
        let year = s[..4].parse::<u32>().unwrap();
        let month = s[5..7].parse::<u32>().unwrap();
        let day = s[8..].parse::<u32>().unwrap();
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let days = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if leap {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        };
        if year == 0 || day == 0 || day > days {
            return Err(err(
                "INVALID_DATE",
                field,
                "Date is outside the Gregorian calendar",
            ));
        }
        Ok(Self { year, month, day })
    }
}
fn required_date(s: &Option<String>, field: &str) -> Result<Date> {
    Date::parse(
        s.as_deref()
            .ok_or_else(|| err("MISSING_FIELD", field, "An explicit date is required"))?,
        field,
    )
}
fn normalize(part: &str, field: &str, required: bool) -> Result<String> {
    if part.len() > 1024 {
        return Err(err(
            "INPUT_TOO_LONG",
            field,
            "A name component is limited to 1024 UTF-8 bytes",
        ));
    }
    let mut out = String::new();
    let mut separator = false;
    for c in part.nfd().filter(|c| !is_combining_mark(*c)) {
        let c = match c {
            'đ' | 'Đ' => 'D',
            _ => c,
        };
        if c.is_ascii_alphabetic() {
            if separator && !out.is_empty() {
                out.push(' ')
            }
            separator = false;
            out.push(c.to_ascii_uppercase());
        } else if c.is_whitespace() || ['\'', '’', '-'].contains(&c) {
            separator = true
        } else {
            return Err(err("UNSUPPORTED_CHARACTER",field,"Only Latin letters with decomposable accents, Vietnamese Đ, spaces, hyphens and apostrophes are supported; transliterate other scripts explicitly"));
        }
    }
    if required && out.is_empty() {
        return Err(err(
            "EMPTY_NAME",
            field,
            "Given name must contain a supported letter",
        ));
    }
    Ok(out)
}
fn normalized_name(r: &Request) -> Result<Name> {
    let n = r.name.as_ref().ok_or_else(|| {
        err(
            "MISSING_FIELD",
            "name",
            "Provide structured given, middle and family name components",
        )
    })?;
    normalize_parts(n, "name")
}
fn normalize_parts(n: &Name, prefix: &str) -> Result<Name> {
    Ok(Name {
        given: normalize(&n.given, &format!("{prefix}.given"), true)?,
        middle: normalize(&n.middle, &format!("{prefix}.middle"), false)?,
        family: normalize(&n.family, &format!("{prefix}.family"), false)?,
    })
}
fn letter(c: u8) -> u32 {
    (c - b'A') as u32 % 9 + 1
}
fn total(s: &str) -> u32 {
    s.bytes().filter(u8::is_ascii_uppercase).map(letter).sum()
}
fn vowel(word: &[u8], i: usize, y: YRule) -> bool {
    match word[i] {
        b'A' | b'E' | b'I' | b'O' | b'U' => true,
        b'Y' => match y {
            YRule::Vowel => true,
            YRule::Consonant => false,
            YRule::Contextual => {
                let ordinary = |c: u8| matches!(c, b'A' | b'E' | b'I' | b'O' | b'U');
                !(i > 0 && ordinary(word[i - 1]) || i + 1 < word.len() && ordinary(word[i + 1]))
            }
        },
        _ => false,
    }
}
fn name_numbers(n: &Name, o: &Options) -> Value {
    let full = format!("{} {} {}", n.given, n.middle, n.family);
    let mut vowels = 0;
    let mut consonants = 0;
    let mut freq = BTreeMap::new();
    for i in 1..=9 {
        freq.insert(i.to_string(), 0u32);
    }
    for word in full.split_whitespace() {
        for (i, c) in word.bytes().enumerate() {
            let value = letter(c);
            *freq.get_mut(&value.to_string()).unwrap() += 1;
            if vowel(word.as_bytes(), i, o.y_rule) {
                vowels += value
            } else {
                consonants += value
            }
        }
    }
    let missing: Vec<u32> = (1..=9).filter(|i| freq[&i.to_string()] == 0).collect();
    let max = freq.values().copied().max().unwrap_or(0);
    let passion: Vec<u32> = (1..=9)
        .filter(|i| max > 0 && freq[&i.to_string()] == max)
        .collect();
    let initials = full
        .split_whitespace()
        .map(|w| letter(w.as_bytes()[0]))
        .sum();
    json!({"normalizedName":n,"expression":reduce(vowels+consonants,o.keep_master_numbers),"soulUrge":reduce(vowels,o.keep_master_numbers),"personality":reduce(consonants,o.keep_master_numbers),"balance":reduce(initials,o.keep_master_numbers),"givenName":reduce(total(&n.given),o.keep_master_numbers),"familyName":reduce(total(&n.family),o.keep_master_numbers),"frequency":freq,"missingNumbers":missing,"hiddenPassion":passion,"subconsciousSelf":9-missing.len(),"cornerstone":n.given.chars().next().unwrap().to_string(),"capstone":n.given.chars().rev().find(|c|c.is_ascii_uppercase()).unwrap().to_string()})
}
fn birth_numbers(d: Date, keep: bool) -> Value {
    let day = reduce(d.day, keep);
    let month = reduce(d.month, keep);
    let year = reduce(d.year, keep);
    let life = reduce(day.value + month.value + year.value, keep);
    let mut grid = BTreeMap::new();
    for i in 1..=9 {
        grid.insert(i.to_string(), 0u32);
    }
    for c in format!("{:04}{:02}{:02}", d.year, d.month, d.day)
        .chars()
        .filter(|c| *c != '0')
    {
        *grid.get_mut(&c.to_string()).unwrap() += 1;
    }
    json!({"lifePath":life,"birthDay":reduce(d.day,keep),"attitude":reduce(d.day+d.month,false),"components":{"day":day,"month":month,"year":year},"birthGrid":grid})
}
fn cycles(d: Date, t: Date, keep: bool) -> Value {
    let day = reduce(d.day, false).value;
    let month = reduce(d.month, false).value;
    let year = reduce(d.year, false).value;
    let lp = reduce(
        reduce(d.day, keep).value + reduce(d.month, keep).value + reduce(d.year, keep).value,
        keep,
    );
    let end = 36 - reduce(lp.value, false).value;
    let p1 = reduce(month + day, keep);
    let p2 = reduce(day + year, keep);
    let p3 = reduce(p1.value + p2.value, keep);
    let p4 = reduce(month + year, keep);
    let c1 = month.abs_diff(day);
    let c2 = day.abs_diff(year);
    let py = reduce(month + day + reduce(t.year, false).value, false);
    let pm = reduce(py.value + t.month, false);
    let pd = reduce(pm.value + t.day, false);
    let uy = reduce(t.year, false);
    let um = reduce(uy.value + t.month, false);
    let ud = reduce(um.value + t.day, false);
    json!({"personalYear":py,"personalMonth":pm,"personalDay":pd,"universalYear":uy,"universalMonth":um,"universalDay":ud,
 "pinnacles":[{"phase":1,"number":p1,"startAge":0,"endAge":end},{"phase":2,"number":p2,"startAge":end+1,"endAge":end+9},{"phase":3,"number":p3,"startAge":end+10,"endAge":end+18},{"phase":4,"number":p4,"startAge":end+19,"endAge":null}],
 "challenges":[reduce(c1,false),reduce(c2,false),reduce(c1.abs_diff(c2),false),reduce(month.abs_diff(year),false)]})
}
fn transit(part: &str, age: u16) -> Value {
    let letters: Vec<u8> = part.bytes().filter(u8::is_ascii_uppercase).collect();
    let duration: u32 = letters.iter().map(|c| letter(*c)).sum();
    if duration == 0 {
        return json!({"letter":null,"value":0});
    }
    let mut position = age as u32 % duration;
    for c in letters {
        let value = letter(c);
        if position < value {
            return json!({"letter":(c as char).to_string(),"value":value});
        }
        position -= value;
    }
    unreachable!()
}
fn essence(n: &Name, d: Date, r: &Request) -> Result<Value> {
    if r.years == 0
        || r.years > 120
        || r.start_age > 120
        || r.start_age as u32 + r.years as u32 > 121
    {
        return Err(err(
            "INVALID_RANGE",
            "years",
            "Use 1..120 years within ages 0..120",
        ));
    }
    let years:Vec<Value>=(r.start_age..r.start_age+r.years).map(|age| {
  let given=transit(&n.given,age);let middle=transit(&n.middle,age);let family=transit(&n.family,age);
  let sum=given["value"].as_u64().unwrap()+middle["value"].as_u64().unwrap()+family["value"].as_u64().unwrap();
  json!({"age":age,"birthYearPlusAge":d.year+age as u32,"given":given,"middle":middle,"family":family,"essence":reduce(sum as u32,r.options.keep_master_numbers)})
 }).collect();
    Ok(json!({"years":years}))
}
pub fn calculate(r: &Request) -> Result<Value> {
    if r.start_age > 120 || r.years == 0 || r.years > 120 {
        return Err(err(
            "INVALID_RANGE",
            "years",
            "Use 1..120 years and startAge within 0..120",
        ));
    }
    if r.system != "pythagorean" {
        return Err(err(
            "UNSUPPORTED_SYSTEM",
            "system",
            "This release supports only pythagorean",
        ));
    }
    let mut data = serde_json::Map::new();
    let mut warnings = vec![];
    let operation = r.operation.as_str();
    if !["profile", "name", "birth", "cycles", "essence"].contains(&operation) {
        return Err(err(
            "UNSUPPORTED_OPERATION",
            "operation",
            "Use profile, name, birth, cycles or essence",
        ));
    }
    let needs_name = ["profile", "name", "essence"].contains(&operation);
    let name = if needs_name {
        Some(normalized_name(r)?)
    } else {
        None
    };
    if let Some(n) = &name {
        if matches!(r.options.y_rule, YRule::Contextual)
            && format!("{}{}{}", n.given, n.middle, n.family).contains('Y')
        {
            warnings.push("Contextual Y is a documented spelling heuristic, not phonetic analysis; choose vowel or consonant explicitly if needed")
        }
    }
    let birth = if operation != "name" {
        Some(required_date(&r.birth_date, "birthDate")?)
    } else {
        None
    };
    if operation == "name" || operation == "profile" {
        data.insert(
            "name".into(),
            name_numbers(name.as_ref().unwrap(), &r.options),
        );
    }
    if operation == "birth" || operation == "profile" {
        data.insert(
            "birth".into(),
            birth_numbers(birth.unwrap(), r.options.keep_master_numbers),
        );
    }
    if operation == "cycles" || operation == "profile" {
        let reference = required_date(&r.reference_date, "referenceDate")?;
        data.insert(
            "cycles".into(),
            cycles(birth.unwrap(), reference, r.options.keep_master_numbers),
        );
    }
    if operation == "essence" || operation == "profile" {
        data.insert(
            "essence".into(),
            essence(name.as_ref().unwrap(), birth.unwrap(), r)?,
        );
    }
    if operation == "profile" {
        let lp = data["birth"]["lifePath"]["value"].as_u64().unwrap() as u32;
        let expression = data["name"]["expression"]["value"].as_u64().unwrap() as u32;
        let soul = data["name"]["soulUrge"]["value"].as_u64().unwrap() as u32;
        let personality = data["name"]["personality"]["value"].as_u64().unwrap() as u32;
        data.insert("derived".into(),json!({"maturity":reduce(lp+expression,r.options.keep_master_numbers),"lifePathExpressionBridge":lp.abs_diff(expression),"soulPersonalityBridge":soul.abs_diff(personality)}));
    }
    Ok(
        json!({"schemaVersion":"1.0","engineVersion":VERSION,"system":"pythagorean","rulesVersion":RULES,"operation":operation,"options":r.options,"data":data,"warnings":warnings}),
    )
}
pub fn calculate_json(input: &str) -> String {
    let result = if input.len() > MAX_REQUEST_BYTES {
        Err(err(
            "REQUEST_TOO_LARGE",
            "request",
            "JSON request exceeds 16384 bytes",
        ))
    } else {
        serde_json::from_str::<WireRequest>(input).map_err(|_|err("INVALID_REQUEST","request","Malformed JSON, duplicate/unknown fields, missing operation or incorrect field types")).and_then(WireRequest::calculate)
    };
    match result {
        Ok(v) => json!({"ok":true,"result":v}).to_string(),
        Err(e) => json!({"ok":false,"error":e}).to_string(),
    }
}
/// Bounded UTF-8 input for C/CLI integrations, including malformed byte streams.
pub fn calculate_bytes(input: &[u8]) -> String {
    if input.len() > MAX_REQUEST_BYTES {
        return serde_json::json!({"ok":false,"error":err("REQUEST_TOO_LARGE","request","JSON request exceeds 16384 bytes")}).to_string();
    }
    match std::str::from_utf8(input) {
        Ok(input) => calculate_json(input),
        Err(_) => serde_json::json!({"ok":false,"error":err("INVALID_UTF8","request","Input must be UTF-8")}).to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reduction_trace() {
        assert_eq!(reduce(49, true).steps, vec![49, 13, 4]);
        assert_eq!(reduce(49, true).karmic_debt, Some(13));
        assert_eq!(reduce(11, true).value, 11);
        assert_eq!(reduce(11, false).value, 2);
        assert_eq!(reduce(0, true).value, 0);
    }
    #[test]
    fn gregorian() {
        for s in [
            "1900-02-29",
            "2000-02-30",
            "0000-01-01",
            "2026-13-01",
            "２０２６-01-01",
            "2026-1-01",
        ] {
            assert!(Date::parse(s, "date").is_err(), "{s}")
        }
        assert!(Date::parse("2000-02-29", "date").is_ok());
        assert!(Date::parse("0001-01-01", "date").is_ok());
        assert!(Date::parse("9999-12-31", "date").is_ok());
    }
    #[test]
    fn normalization() {
        assert_eq!(normalize("Đặng Thị", "name", true).unwrap(), "DANG THI");
        assert_eq!(normalize("A\u{301}n", "name", true).unwrap(), "AN");
        for s in ["123", "王明", "---", ""] {
            assert!(normalize(s, "name", true).is_err())
        }
    }
    #[test]
    fn rejects_unknown() {
        for s in [
            r#"{"operation":"name","oops":1}"#,
            r#"{"operation":"birth","operation":"name"}"#,
            r#"{"operation":"birth","system":"chaldean"}"#,
        ] {
            assert_eq!(
                serde_json::from_str::<Value>(&calculate_json(s)).unwrap()["ok"],
                false
            )
        }
    }
    #[test]
    fn y_applies_per_occurrence() {
        let word = b"YAY";
        assert!(!vowel(word, 0, YRule::Contextual));
        assert!(!vowel(word, 2, YRule::Contextual));
        assert!(vowel(b"LY", 1, YRule::Contextual));
        assert!(!vowel(b"LY", 1, YRule::Consonant));
        assert!(vowel(b"YEN", 0, YRule::Vowel));
    }
    #[test]
    fn transit_is_bounded() {
        assert_eq!(transit("", 120)["value"], 0);
        assert_eq!(transit("A", 120)["letter"], "A");
        assert_eq!(transit("B", 0)["value"], 2);
    }
}
