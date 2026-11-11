//! Opt-in arithmetic extensions. Legacy calculations and the public Request stay unchanged.
use crate::{
    birth_numbers, calculate, err, name_numbers, normalize, normalized_name, reduce, required_date,
    total, Name, Request, Result, YRule,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

pub const EXTENSION_RULES: &str = "pythagorean-7mlabs-extensions-1.0";

/// JSON callers enable these additions with `"extensions": {}`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Extensions {
    #[serde(default = "crate::yes")]
    pub reduced_values: bool,
    #[serde(default = "crate::yes")]
    pub indicators: bool,
    pub current_name: Option<Name>,
}
impl Default for Extensions {
    fn default() -> Self {
        Self {
            reduced_values: true,
            indicators: true,
            current_name: None,
        }
    }
}

fn current_name(n: &Name) -> Result<Name> {
    Ok(Name {
        given: normalize(&n.given, "extensions.currentName.given", true)?,
        middle: normalize(&n.middle, "extensions.currentName.middle", false)?,
        family: normalize(&n.family, "extensions.currentName.family", false)?,
    })
}

// Neutral grid coordinates; no psychological or predictive labels are inferred.
fn axes(grid: &Value) -> Value {
    let triples = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [1, 4, 7],
        [2, 5, 8],
        [3, 6, 9],
        [1, 5, 9],
        [3, 5, 7],
    ];
    let rows: Vec<Value> = triples
        .iter()
        .map(|digits| {
            let occupied = digits
                .iter()
                .filter(|n| grid[n.to_string()].as_u64().unwrap() > 0)
                .count();
            json!({"digits":digits,"status":match occupied {
                0 => "absent", 3 => "present", _ => "partial"
            }})
        })
        .collect();
    json!(rows)
}

pub(crate) fn enrich_numbers(value: &mut Value) {
    match value {
        Value::Array(values) => values.iter_mut().for_each(enrich_numbers),
        Value::Object(object) => {
            for child in object.values_mut() {
                enrich_numbers(child);
            }
            if object.contains_key("root")
                && object.contains_key("steps")
                && object.contains_key("isMaster")
                && object.contains_key("karmicDebt")
            {
                if let Some(n) = object.get("value").and_then(Value::as_u64) {
                    object.insert("reducedValue".into(), json!(reduce(n as u32, false).value));
                }
            }
        }
        _ => {}
    }
}

/// Preserve all base values. Additional fields are only returned through this API
/// or when a JSON request explicitly supplies an extensions object.
pub fn calculate_with_extensions(r: &Request, e: &Extensions) -> Result<Value> {
    let mut output = calculate(r)?;
    let current = match &e.current_name {
        Some(name) => {
            if !e.indicators || !["name", "profile"].contains(&r.operation.as_str()) {
                return Err(err(
                    "INVALID_EXTENSION",
                    "extensions.currentName",
                    "currentName requires indicators=true and operation name or profile",
                ));
            }
            Some(current_name(name)?)
        }
        None => None,
    };
    output["schemaVersion"] = json!("1.1");
    output["extensions"] = json!({
        "rulesVersion":EXTENSION_RULES,
        "reducedValues":e.reduced_values,
        "indicators":e.indicators,
    });
    if e.indicators {
        let mut extra = Map::new();
        let data = &output["data"];
        if data.get("birth").is_some() {
            extra.insert("birthGridAxes".into(), axes(&data["birth"]["birthGrid"]));
        }
        if data.get("name").is_some() {
            extra.insert("birthNameGridAxes".into(), axes(&data["name"]["frequency"]));
        }
        if let Some(name) = &current {
            extra.insert("currentName".into(), name_numbers(name, &r.options));
            if matches!(r.options.y_rule, YRule::Contextual)
                && format!("{}{}{}", name.given, name.middle, name.family).contains('Y')
            {
                output["warnings"].as_array_mut().unwrap().push(json!(
                    "Contextual Y in currentName is a documented spelling heuristic, not phonetic analysis; choose vowel or consonant explicitly if needed"
                ));
            }
        }
        if output["data"].get("cycles").is_some() {
            let birth = required_date(&r.birth_date, "birthDate")?;
            let numbers = birth_numbers(birth, r.options.keep_master_numbers);
            let lp = numbers["lifePath"]["value"].as_u64().unwrap() as u32;
            let end = 36 - reduce(lp, false).value;
            extra.insert("periodCycles".into(), json!([
                {"phase":1,"source":"birthMonth","number":numbers["components"]["month"],"startAge":0,"endAge":end},
                {"phase":2,"source":"birthDay","number":numbers["components"]["day"],"startAge":end+1,"endAge":end+27},
                {"phase":3,"source":"birthYear","number":numbers["components"]["year"],"startAge":end+28,"endAge":null},
            ]));
        }
        if r.operation == "profile" {
            let name = normalized_name(r)?;
            let birth = required_date(&r.birth_date, "birthDate")?;
            extra.insert(
                "rationalThought".into(),
                json!({
                    "nameSource":"birthName.given",
                    "number":reduce(total(&name.given)+birth.day,false),
                }),
            );
            let name_grid = if current.is_some() {
                extra["currentName"]["frequency"].clone()
            } else {
                output["data"]["name"]["frequency"].clone()
            };
            let birth_grid = &output["data"]["birth"]["birthGrid"];
            let mut combined = Map::new();
            for n in 1..=9 {
                let key = n.to_string();
                combined.insert(
                    key.clone(),
                    json!(birth_grid[&key].as_u64().unwrap() + name_grid[&key].as_u64().unwrap()),
                );
            }
            let combined = Value::Object(combined);
            extra.insert(
                "combinedGrid".into(),
                json!({
                    "nameSource":if current.is_some() {"currentName"} else {"birthName"},
                    "nameFrequency":name_grid,
                    "frequency":combined,
                    "axes":axes(&combined),
                }),
            );
        }
        if !extra.is_empty() {
            output["data"]["extended"] = Value::Object(extra);
        }
    }
    if e.reduced_values {
        enrich_numbers(&mut output["data"]);
    }
    Ok(output)
}
