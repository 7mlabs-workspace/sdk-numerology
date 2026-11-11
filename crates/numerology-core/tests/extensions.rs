use numerology_core::{calculate_json, calculate_with_extensions, Extensions, Request};
use serde_json::{json, Value};

fn calc(request: Value) -> Value {
    serde_json::from_str(&calculate_json(&request.to_string())).unwrap()
}
fn sample() -> Value {
    // Synthetic fixture with a retained 33, a compound 10, and explicit name parts.
    json!({"operation":"profile","name":{"given":"HENIT","middle":"RUH UTAN","family":"IUB"},
      "birthDate":"2006-02-08","referenceDate":"2026-11-11","years":1})
}
fn strip_reduced(v: &mut Value) {
    match v {
        Value::Object(o) => {
            o.remove("reducedValue");
            o.values_mut().for_each(strip_reduced);
        }
        Value::Array(a) => a.iter_mut().for_each(strip_reduced),
        _ => {}
    }
}

#[test]
fn opt_in_preserves_all_legacy_values() {
    for op in ["profile", "name", "birth", "cycles", "essence"] {
        let mut q = sample();
        q["operation"] = json!(op);
        let old = calc(q.clone());
        assert_eq!(old["ok"], true);
        assert!(old["result"].get("extensions").is_none());
        assert_eq!(old["result"]["schemaVersion"], "1.0");
        q["extensions"] = json!({});
        let mut extended = calc(q);
        assert_eq!(extended["ok"], true);
        assert_eq!(extended["result"]["schemaVersion"], "1.1");
        extended["result"]
            .as_object_mut()
            .unwrap()
            .remove("extensions");
        extended["result"]["schemaVersion"] = json!("1.0");
        extended["result"]["data"]
            .as_object_mut()
            .unwrap()
            .remove("extended");
        strip_reduced(&mut extended);
        assert_eq!(extended, old, "{op}");
    }
}

#[test]
fn displays_both_master_and_single_digit_without_changing_the_path() {
    let mut q = sample();
    q["extensions"] = json!({});
    let v = calc(q.clone());
    let n = &v["result"]["data"]["name"];
    assert_eq!(n["soulUrge"]["value"], 33);
    assert_eq!(n["soulUrge"]["reducedValue"], 6);
    assert_eq!(n["soulUrge"]["steps"], json!([33]));
    assert_eq!(n["expression"]["value"], 11);
    assert_eq!(n["expression"]["reducedValue"], 2);
    let fourth = &v["result"]["data"]["cycles"]["pinnacles"][3]["number"];
    assert_eq!(fourth["root"], 10);
    assert_eq!(fourth["value"], 1);
    assert_eq!(fourth["reducedValue"], 1);
    assert_eq!(fourth["isMaster"], false);
    q["options"] = json!({"keepMasterNumbers":false});
    let off = calc(q);
    assert_eq!(off["result"]["data"]["name"]["soulUrge"]["value"], 6);
    assert_eq!(off["result"]["data"]["name"]["soulUrge"]["isMaster"], false);
}

#[test]
fn reduced_values_cover_all_masters_and_zero_without_adding_steps() {
    for (name, expected, reduced) in [("AAAAABBB", 11, 2), ("IID", 22, 4), ("IIIF", 33, 6)] {
        let v = calc(json!({"operation":"name","name":{"given":name},"extensions":{}}));
        let number = &v["result"]["data"]["name"]["expression"];
        assert_eq!(number["value"], expected, "{name}");
        assert_eq!(number["reducedValue"], reduced, "{name}");
        assert_eq!(number["steps"], json!([expected]));
    }
    let v = calc(json!({"operation":"name","name":{"given":"BC"},"extensions":{}}));
    assert_eq!(v["result"]["data"]["name"]["soulUrge"]["reducedValue"], 0);
    assert_eq!(v["result"]["data"]["name"]["soulUrge"]["isMaster"], false);
}

#[test]
fn known_periods_and_age_bands_are_contiguous_and_unbounded_at_the_end() {
    let mut q = sample();
    q["operation"] = json!("cycles");
    q["extensions"] = json!({});
    let v = calc(q);
    let p = v["result"]["data"]["extended"]["periodCycles"]
        .as_array()
        .unwrap();
    assert_eq!(
        p.iter()
            .map(|p| p["number"]["value"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        vec![2, 8, 8]
    );
    assert_eq!(p[0]["startAge"], 0);
    assert_eq!(p[0]["endAge"], 27);
    assert_eq!(p[1]["startAge"], 28);
    assert_eq!(p[1]["endAge"], 54);
    assert_eq!(p[2]["startAge"], 55);
    assert_eq!(p[2]["endAge"], Value::Null);
    let master = calc(
        json!({"operation":"cycles","birthDate":"1975-11-25","referenceDate":"2026-01-01","extensions":{}}),
    );
    let p = &master["result"]["data"]["extended"]["periodCycles"];
    assert_eq!(p[0]["number"]["value"], 11);
    assert_eq!(p[0]["number"]["reducedValue"], 2);
    assert_eq!(p[2]["number"]["value"], 22);
    assert_eq!(p[2]["number"]["reducedValue"], 4);
}

#[test]
fn rational_thought_uses_birth_given_name_and_never_keeps_a_master() {
    let v = calc(
        json!({"operation":"profile","name":{"given":"Thomas"},"birthDate":"2000-01-03", "referenceDate":"2026-01-01","years":1,"extensions":{}}),
    );
    let thought = &v["result"]["data"]["extended"]["rationalThought"];
    // THOMAS = 2+8+6+4+1+1 = 22; plus day 3 = 25 -> 7.
    assert_eq!(thought["nameSource"], "birthName.given");
    assert_eq!(thought["number"]["root"], 25);
    assert_eq!(thought["number"]["value"], 7);
    let v = calc(
        json!({"operation":"profile","name":{"given":"B"},"birthDate":"2000-01-09", "referenceDate":"2026-01-01","years":1,"extensions":{}}),
    );
    assert_eq!(
        v["result"]["data"]["extended"]["rationalThought"]["number"]["steps"],
        json!([11, 2])
    );
}

#[test]
fn current_name_is_separate_and_combined_grid_is_an_exact_sum() {
    let mut q = sample();
    let old = calc(q.clone());
    q["extensions"] = json!({"currentName":{"given":"PAM NETIH"}});
    let v = calc(q);
    assert_eq!(v["ok"], true);
    let ext = &v["result"]["data"]["extended"];
    assert_eq!(ext["currentName"]["soulUrge"]["value"], 6);
    assert_eq!(ext["currentName"]["expression"]["value"], 5);
    assert_eq!(ext["currentName"]["personality"]["value"], 8);
    assert_eq!(ext["combinedGrid"]["nameSource"], "currentName");
    assert_eq!(
        ext["combinedGrid"]["frequency"],
        json!({"1":1,"2":3,"3":0,"4":1,"5":2,"6":1,"7":1,"8":2,"9":1})
    );
    let mut birth_name = v["result"]["data"]["name"].clone();
    strip_reduced(&mut birth_name);
    assert_eq!(birth_name, old["result"]["data"]["name"]);
    // The birth-name rational thought is 29+8 -> 1, not the comparison website's 4.
    assert_eq!(ext["rationalThought"]["number"]["value"], 1);
    let mut q = sample();
    q["extensions"] = json!({});
    let default = calc(q);
    assert_eq!(
        default["result"]["data"]["extended"]["combinedGrid"]["nameSource"],
        "birthName"
    );
    assert!(default["result"]["data"]["extended"]
        .get("currentName")
        .is_none());
}

#[test]
fn grid_axes_include_present_absent_and_partial_without_interpretation() {
    let v = calc(json!({"operation":"birth","birthDate":"1987-02-09","extensions":{}}));
    let a = v["result"]["data"]["extended"]["birthGridAxes"]
        .as_array()
        .unwrap();
    assert_eq!(a.len(), 8);
    for (digits, status) in [
        (json!([7, 8, 9]), "present"),
        (json!([4, 5, 6]), "absent"),
        (json!([1, 2, 3]), "partial"),
    ] {
        assert_eq!(
            a.iter().find(|x| x["digits"] == digits).unwrap()["status"],
            status
        );
    }
}

#[test]
fn extension_flags_do_not_silently_ignore_current_name_or_add_unrequested_fields() {
    let mut q = sample();
    q["extensions"] = json!({"reducedValues":false,"indicators":false});
    let v = calc(q.clone());
    assert!(v["result"]["data"].get("extended").is_none());
    assert!(v["result"]["data"]["name"]["soulUrge"]
        .get("reducedValue")
        .is_none());
    q["extensions"]["currentName"] = json!({"given":"A"});
    assert_eq!(calc(q)["error"]["code"], "INVALID_EXTENSION");
    for op in ["birth", "cycles", "essence"] {
        let mut q = sample();
        q["operation"] = json!(op);
        q["extensions"] = json!({"currentName":{"given":"A"}});
        assert_eq!(calc(q)["error"]["field"], "extensions.currentName");
    }
}

#[test]
fn extended_input_keeps_strict_validation_and_reports_current_name_fields() {
    for raw in [
        r#"{"operation":"name","name":{"given":"A"},"extensions":{"reducedValues":true,"reducedValues":false}}"#,
        r#"{"operation":"name","name":{"given":"A"},"extensions":{},"extensions":{}}"#,
        r#"{"operation":"name","name":{"given":"A"},"extensions":{"typo":true}}"#,
        r#"{"operation":"name","name":{"given":"A"},"extensions":{"reducedValues":"yes"}}"#,
        r#"{"operation":"name","name":{"given":"A"},"extensions":{"currentName":{"given":"B","alias":"C"}}}"#,
    ] {
        let v: Value = serde_json::from_str(&calculate_json(raw)).unwrap();
        assert_eq!(v["error"]["code"], "INVALID_REQUEST", "{raw}");
    }
    for name in ["王明", "123", "---", ""] {
        let v = calc(
            json!({"operation":"name","name":{"given":"A"},"extensions":{"currentName":{"given":name}}}),
        );
        assert_eq!(v["ok"], false);
        assert_eq!(v["error"]["field"], "extensions.currentName.given");
    }
    let v = calc(
        json!({"operation":"name","name":{"given":"A"},"extensions":{"currentName":{"given":"Đặng Ly"}}}),
    );
    assert_eq!(
        v["result"]["data"]["extended"]["currentName"]["normalizedName"]["given"],
        "DANG LY"
    );
    assert_eq!(v["result"]["warnings"].as_array().unwrap().len(), 1);
}

#[test]
fn public_rust_request_struct_and_legacy_calculate_stay_source_compatible() {
    let request = Request {
        operation: "birth".into(),
        system: "pythagorean".into(),
        name: None,
        birth_date: Some("2000-02-29".into()),
        reference_date: None,
        options: Default::default(),
        start_age: 0,
        years: 100,
    };
    assert_eq!(
        numerology_core::calculate(&request).unwrap()["schemaVersion"],
        "1.0"
    );
    assert_eq!(
        calculate_with_extensions(&request, &Extensions::default()).unwrap()["schemaVersion"],
        "1.1"
    );
}
