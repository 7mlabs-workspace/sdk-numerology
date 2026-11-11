//! Independent name calculations and caller-selected composite name scopes.
use crate::{
    calculate_with_extensions, err, name_numbers, normalize_parts, normalized_name, reduce, total,
    Date, Extensions, Name, Options, Request, Result, YRule, RULES, VERSION,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const NAMES_RULES: &str = "pythagorean-7mlabs-names-1.0";
const MAX_NAMES: usize = 32;
const MAX_COMBINATIONS: usize = 32;
const MAX_SOURCES: usize = 32;
const MAX_COMBINED_BYTES: usize = 4096;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NameInput {
    pub id: String,
    pub name: Name,
}
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NamePart {
    #[default]
    Full,
    Given,
    Middle,
    Family,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NameSource {
    pub name_id: String,
    #[serde(default)]
    pub part: NamePart,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NameCombination {
    pub id: String,
    pub sources: Vec<NameSource>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NameSet {
    #[serde(default)]
    pub names: Vec<NameInput>,
    #[serde(default)]
    pub combinations: Vec<NameCombination>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamesRequest {
    pub names: Vec<NameInput>,
    #[serde(default)]
    pub combinations: Vec<NameCombination>,
    pub birth_date: Option<String>,
    #[serde(default = "crate::system")]
    pub system: String,
    #[serde(default)]
    pub options: Options,
}

fn validate_id(id: &str, field: &str) -> Result<()> {
    let valid = !id.is_empty()
        && id.len() <= 64
        && id.as_bytes()[0].is_ascii_alphanumeric()
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b));
    if valid {
        Ok(())
    } else {
        Err(err(
            "INVALID_ID",
            field,
            "Use 1..64 ASCII letters/digits/underscore/dot/hyphen, starting with a letter or digit",
        ))
    }
}

fn selected(n: &Name, part: NamePart) -> (String, u8) {
    let fields = [(&n.given, 1), (&n.middle, 2), (&n.family, 4)];
    let mask = match part {
        NamePart::Full => 7,
        NamePart::Given => 1,
        NamePart::Middle => 2,
        NamePart::Family => 4,
    };
    let mut actual_mask = 0;
    let text = fields
        .iter()
        .filter_map(|(text, flag)| {
            if mask & flag != 0 && !text.is_empty() {
                actual_mask |= flag;
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    (text, actual_mask)
}

fn with_thought(mut entry: Value, name: &Name, birth: Option<Date>, scope: &str) -> Value {
    if let Some(birth) = birth {
        entry["rationalThought"] = json!({
            "nameSource":scope,
            "number":reduce(total(&name.given)+birth.day,false),
        });
    }
    entry
}

fn calculate_set(
    entries: &[NameInput],
    groups: &[NameCombination],
    options: &Options,
    birth: Option<Date>,
    birth_name: Option<Name>,
    reduced_values: bool,
    prefix: &str,
) -> Result<(Value, Vec<String>)> {
    let count = entries.len() + usize::from(birth_name.is_some());
    if count == 0 || count > MAX_NAMES {
        return Err(err(
            "INVALID_RANGE",
            &format!("{prefix}names"),
            "Use 1..32 input names including the injected birth name in profiles",
        ));
    }
    if groups.len() > MAX_COMBINATIONS {
        return Err(err(
            "INVALID_RANGE",
            &format!("{prefix}combinations"),
            "Use at most 32 explicit combinations",
        ));
    }
    let mut ids = BTreeSet::new();
    let mut inputs = Vec::new();
    let has_birth = birth_name.is_some();
    if let Some(name) = birth_name {
        ids.insert("birth".to_owned());
        inputs.push(("birth".to_owned(), name, "profile.birthName"));
    }
    for (i, input) in entries.iter().enumerate() {
        let field = format!("{prefix}names[{i}]");
        validate_id(&input.id, &format!("{field}.id"))?;
        if has_birth && input.id == "birth" {
            return Err(err(
                "RESERVED_NAME_ID",
                &format!("{field}.id"),
                "birth is reserved for the profile's birth name",
            ));
        }
        if !ids.insert(input.id.clone()) {
            return Err(err(
                "DUPLICATE_ID",
                &format!("{field}.id"),
                "Each input and combination id must be unique",
            ));
        }
        inputs.push((
            input.id.clone(),
            normalize_parts(&input.name, &format!("{field}.name"))?,
            "input",
        ));
    }
    let lookup: BTreeMap<_, _> = inputs
        .iter()
        .map(|(id, name, _)| (id.as_str(), name))
        .collect();
    let mut warnings = vec![];
    let mut names = vec![];
    for (id, name, source) in &inputs {
        if matches!(options.y_rule, YRule::Contextual)
            && format!("{}{}{}", name.given, name.middle, name.family).contains('Y')
        {
            warnings.push(format!("Name {id}: contextual Y is a spelling heuristic, not phonetic analysis; choose vowel or consonant explicitly if needed"));
        }
        names.push(with_thought(
            json!({"id":id,"source":source,"numbers":name_numbers(name,options)}),
            name,
            birth,
            "given",
        ));
    }
    let mut combinations = vec![];
    for (i, group) in groups.iter().enumerate() {
        let field = format!("{prefix}combinations[{i}]");
        validate_id(&group.id, &format!("{field}.id"))?;
        if !ids.insert(group.id.clone()) {
            return Err(err(
                "DUPLICATE_ID",
                &format!("{field}.id"),
                "Each input and combination id must be unique",
            ));
        }
        if group.sources.len() < 2 || group.sources.len() > MAX_SOURCES {
            return Err(err(
                "INVALID_RANGE",
                &format!("{field}.sources"),
                "A combination requires 2..32 selected sources",
            ));
        }
        let mut texts = BTreeSet::new();
        let mut used_parts: BTreeMap<String, u8> = BTreeMap::new();
        let mut resolved = vec![];
        let mut joined = vec![];
        for (j, source) in group.sources.iter().enumerate() {
            let source_field = format!("{field}.sources[{j}]");
            let name = lookup.get(source.name_id.as_str()).ok_or_else(|| {
                err(
                    "UNKNOWN_NAME_ID",
                    &format!("{source_field}.nameId"),
                    "Sources must reference input names; referencing combinations is not supported",
                )
            })?;
            let (text, mask) = selected(name, source.part);
            if text.is_empty() {
                return Err(err(
                    "EMPTY_NAME_PART",
                    &format!("{source_field}.part"),
                    "The selected name part is empty",
                ));
            }
            let key = format!("{}\0{}\0{}", name.given, name.middle, name.family);
            let seen = used_parts.entry(key).or_default();
            if *seen & mask != 0 || !texts.insert(text.clone()) {
                return Err(err("DUPLICATE_NAME_SOURCE", &source_field, "A combination cannot repeat normalized text or overlap parts of the same normalized name"));
            }
            *seen |= mask;
            joined.push(text.clone());
            resolved.push(json!({"nameId":source.name_id,"part":source.part,"text":text}));
        }
        let text = joined.join(" ");
        if text.len() > MAX_COMBINED_BYTES {
            return Err(err(
                "INPUT_TOO_LONG",
                &format!("{field}.sources"),
                "Selected normalized name text is limited to 4096 UTF-8 bytes",
            ));
        }
        let name = Name {
            given: text,
            middle: String::new(),
            family: String::new(),
        };
        combinations.push(with_thought(
            json!({
                "id":group.id,"method":"concatenateSelectedParts","reduction":"wholeNameSum",
                "sources":resolved,"numbers":name_numbers(&name,options),
            }),
            &name,
            birth,
            "selectedText",
        ));
    }
    let mut result = json!({"rulesVersion":NAMES_RULES,"names":names,"combinations":combinations});
    if reduced_values {
        crate::extensions::enrich_numbers(&mut result);
    }
    Ok((result, warnings))
}

/// Independent names and optional explicit composites; no automatic merging.
pub fn calculate_names(r: &NamesRequest) -> Result<Value> {
    if r.system != "pythagorean" {
        return Err(err(
            "UNSUPPORTED_SYSTEM",
            "system",
            "This release supports only pythagorean",
        ));
    }
    let birth = r
        .birth_date
        .as_deref()
        .map(|s| Date::parse(s, "birthDate"))
        .transpose()?;
    let (data, warnings) =
        calculate_set(&r.names, &r.combinations, &r.options, birth, None, true, "")?;
    Ok(
        json!({"schemaVersion":"1.1","engineVersion":VERSION,"system":"pythagorean",
        "rulesVersion":RULES,"namesRulesVersion":NAMES_RULES,"operation":"names","options":r.options,
        "data":{"names":data},"warnings":warnings}),
    )
}

/// Add a name set to a profile without changing its base name, currentName or grids.
pub fn calculate_with_name_set(r: &Request, e: &Extensions, set: &NameSet) -> Result<Value> {
    let mut output = calculate_with_extensions(r, e)?;
    if r.operation != "profile" || !e.indicators {
        return Err(err(
            "INVALID_EXTENSION",
            "extensions.nameSet",
            "nameSet requires operation profile and indicators=true",
        ));
    }
    let birth_name = normalized_name(r)?;
    let birth = crate::required_date(&r.birth_date, "birthDate")?;
    let (data, warnings) = calculate_set(
        &set.names,
        &set.combinations,
        &r.options,
        Some(birth),
        Some(birth_name),
        e.reduced_values,
        "extensions.nameSet.",
    )?;
    output["data"]["extended"]["nameSet"] = data;
    output["extensions"]["namesRulesVersion"] = json!(NAMES_RULES);
    output["warnings"]
        .as_array_mut()
        .unwrap()
        .extend(warnings.into_iter().map(Value::String));
    Ok(output)
}
