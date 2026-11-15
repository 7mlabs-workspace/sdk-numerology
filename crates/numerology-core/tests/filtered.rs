use numerology_core::{calculate_filtered_bytes, calculate_filtered_json, calculate_json};
use serde_json::{json, Value};

fn full(q: &Value) -> Value {
    serde_json::from_str::<Value>(&calculate_json(&q.to_string())).unwrap()["result"].clone()
}
fn filtered(q: &Value) -> Value {
    let v: Value = serde_json::from_str(&calculate_filtered_json(&q.to_string())).unwrap();
    assert_eq!(v["ok"], true, "{v}");
    v["result"].clone()
}
fn profile() -> Value {
    json!({"operation":"profile","name":{"given":"Ada","middle":"Thị","family":"Đặng"},
        "birthDate":"1980-11-22","referenceDate":"2026-11-13","years":3,
        "extensions":{"currentName":{"given":"Ý"},"nameChallenges":{"nameOrder":"familyMiddleGiven"},
        "nameSet":{"names":[{"id":"Ada_1980_secret","name":{"given":"Mập"}}],
        "combinations":[{"id":"secret_group","sources":[{"nameId":"birth","part":"given"},{"nameId":"Ada_1980_secret"}]}]}}})
}
fn digital_root(mut n: u64) -> u64 {
    while n >= 10 {
        n = n.to_string().bytes().map(|b| u64::from(b - b'0')).sum();
    }
    n
}
fn compare_numbers(original: &Value, output: &Value) -> usize {
    match output {
        Value::Object(object) => {
            if let Some(status) = object.get("traceStatus") {
                assert_eq!(output["value"], original["value"]);
                assert_eq!(output["isMaster"], original["isMaster"]);
                assert_eq!(output["karmicDebt"], original["karmicDebt"]);
                assert_eq!(
                    output["reducedValue"],
                    digital_root(original["value"].as_u64().unwrap())
                );
                if status == "included" {
                    assert_eq!(output["root"], original["root"]);
                    assert_eq!(output["steps"], original["steps"]);
                    assert_eq!(output["steps"][0], output["root"]);
                    assert_eq!(
                        output["steps"].as_array().unwrap().last(),
                        Some(&output["value"])
                    );
                } else {
                    assert_eq!(status, "withheld");
                    assert!(output.get("root").is_none());
                    assert!(output.get("steps").is_none());
                    assert_eq!(output["traceWithheldReason"], "birthDateInference");
                }
                1
            } else {
                object
                    .iter()
                    .map(|(k, v)| compare_numbers(&original[k], v))
                    .sum()
            }
        }
        Value::Array(values) => values
            .iter()
            .enumerate()
            .map(|(i, v)| compare_numbers(&original[i], v))
            .sum(),
        _ => 0,
    }
}
fn no_sensitive_fields(value: &Value) {
    match value {
        Value::Object(o) => {
            for (key, v) in o {
                assert!(
                    ![
                        "normalizedName",
                        "cornerstone",
                        "capstone",
                        "orderedText",
                        "letter",
                        "first",
                        "last",
                        "text",
                        "birthDate",
                        "components",
                        "birthDay",
                        "birthGrid",
                        "periodCycles",
                        "birthGridAxes",
                        "combinedGrid",
                        "birthYearPlusAge"
                    ]
                    .contains(&key.as_str()),
                    "forbidden key {key}"
                );
                no_sensitive_fields(v);
            }
        }
        Value::Array(values) => values.iter().for_each(no_sensitive_fields),
        _ => (),
    }
}

#[test]
fn filtered_profile_removes_direct_and_derived_source_fields() {
    let q = profile();
    let a = full(&q);
    let b = filtered(&q);
    assert_eq!(b["schemaVersion"], "filtered-1.0");
    assert_eq!(b["outputPolicy"]["anonymous"], false);
    assert_eq!(b["outputPolicy"]["referenceDateMayBeRecoverable"], true);
    no_sensitive_fields(&b);
    for needle in [
        "ADA",
        "DANG",
        "1980",
        "Ada_1980_secret",
        "secret_group",
        "MAP",
    ] {
        assert!(!b.to_string().contains(needle), "leaked {needle}");
    }
    assert!(compare_numbers(&a["data"], &b["data"]) > 40);
    assert_eq!(
        full(&q),
        a,
        "filtered call must not change subsequent full output"
    );
    assert!(b["data"]["essence"]["years"][0].get("given").is_none());
    assert!(a["data"]["birth"]["components"].is_object());
}

#[test]
fn reconstructed_dob_paths_are_present_only_in_full_output() {
    let q = profile();
    let a = full(&q);
    let d = &a["data"];
    let day = d["extended"]["rationalThought"]["number"]["root"]
        .as_u64()
        .unwrap()
        - d["name"]["givenName"]["root"].as_u64().unwrap();
    assert_eq!(day, 22);
    assert_eq!(d["birth"]["attitude"]["root"].as_u64().unwrap() - day, 11);
    let year = d["essence"]["years"][0]["birthYearPlusAge"]
        .as_u64()
        .unwrap()
        - d["essence"]["years"][0]["age"].as_u64().unwrap();
    assert_eq!(year, 1980);
    let b = filtered(&q);
    for path in [
        "/data/birth/attitude/root",
        "/data/birth/attitude/steps",
        "/data/extended/rationalThought/number/root",
        "/data/extended/rationalThought/number/steps",
        "/data/extended/nameSet/names/0/rationalThought/number/root",
        "/data/extended/nameSet/combinations/0/rationalThought/number/steps",
    ] {
        assert!(b.pointer(path).is_none(), "{path}");
    }
    for path in [
        "/data/birth/attitude",
        "/data/extended/rationalThought/number",
    ] {
        assert_eq!(
            b.pointer(path).unwrap()["value"],
            a.pointer(path).unwrap()["value"]
        );
    }
    let group = &d["extended"]["nameSet"]["combinations"][0];
    assert_eq!(
        group["rationalThought"]["number"]["root"].as_u64().unwrap()
            - group["numbers"]["expression"]["root"].as_u64().unwrap(),
        22
    );
}

#[test]
fn masters_raw_totals_steps_and_zero_remain_exact() {
    for root in [11, 22, 33, 29, 49] {
        for keep in [false, true] {
            let q = json!({"operation":"name","name":{"given":"A".repeat(root)},"options":{"keepMasterNumbers":keep}});
            let a = full(&q);
            let b = filtered(&q);
            compare_numbers(&a["data"], &b["data"]);
            let n = &b["data"]["name"]["expression"];
            assert_eq!(n["root"], root);
            if keep && [11, 22, 33].contains(&root) {
                assert_eq!(n["value"], root);
                assert_eq!(n["isMaster"], true);
            }
            if root == 49 {
                assert_eq!(n["steps"], json!([49, 13, 4]));
                assert_eq!(n["karmicDebt"], 13);
            }
        }
    }
    for (given, root, steps, value, reduced, master) in [
        ("AIIIIA", 38, json!([38, 11]), 11, 2, true),
        ("IIIAI", 37, json!([37, 10, 1]), 1, 1, false),
        ("B", 2, json!([2]), 2, 2, false),
    ] {
        let q = json!({"operation":"name","name":{"given":given}});
        let b = filtered(&q);
        let n = &b["data"]["name"]["expression"];
        assert_eq!(
            (
                n["root"].as_u64().unwrap(),
                n["value"].as_u64().unwrap(),
                n["reducedValue"].as_u64().unwrap()
            ),
            (root, value, reduced)
        );
        assert_eq!(n["steps"], steps);
        assert_eq!(n["isMaster"], master);
        compare_numbers(&full(&q)["data"], &b["data"]);
    }
    for (given, expected) in [("BBG", 11), ("VV", 8), ("IIII", 9)] {
        let q = json!({"operation":"name","name":{"given":given},"options":{"keepMasterNumbers":false}});
        let b = filtered(&q);
        compare_numbers(&full(&q)["data"], &b["data"]);
        assert!(!b["data"]["name"]["expression"]["isMaster"]
            .as_bool()
            .unwrap());
        assert_eq!(
            b["data"]["name"]["expression"]["value"],
            digital_root(expected)
        );
    }
    let b = filtered(&json!({"operation":"name","name":{"given":"B"}}));
    assert_eq!(b["data"]["name"]["soulUrge"]["root"], 0);
    assert_eq!(b["data"]["name"]["soulUrge"]["steps"], json!([0]));
    assert_eq!(b["data"]["name"]["familyName"]["reducedValue"], 0);
}

#[test]
fn names_ids_and_sources_are_consistently_remapped_and_warnings_do_not_echo_them() {
    let q = json!({"operation":"names","birthDate":"2000-01-01","names":[
        {"id":"RealName_2000_01_01","name":{"given":"Ý"}},
        {"id":"n1","name":{"given":"Ada"}}],"combinations":[
        {"id":"PrivateNickname","sources":[{"nameId":"n1"},{"nameId":"RealName_2000_01_01"}]}]});
    let a = full(&q);
    let b = filtered(&q);
    assert_eq!(b["data"]["names"]["names"][0]["id"], "n1");
    assert_eq!(b["data"]["names"]["names"][1]["id"], "n2");
    assert_eq!(b["data"]["names"]["combinations"][0]["id"], "c1");
    let sources = &b["data"]["names"]["combinations"][0]["sources"];
    assert_eq!(sources[0]["nameId"], "n2");
    assert_eq!(sources[1]["nameId"], "n1");
    for marker in ["RealName_2000_01_01", "PrivateNickname", "ADA", "2000"] {
        assert!(!b.to_string().contains(marker));
    }
    assert!(!b["warnings"].as_array().unwrap().is_empty());
    assert_eq!(b["warnings"][0]["code"], "CONTEXTUAL_Y_HEURISTIC");
    no_sensitive_fields(&b);
    compare_numbers(&a["data"], &b["data"]);
    let mut q2 = q.clone();
    q2["names"][0]["id"] = json!("DifferentId");
    q2["combinations"][0]["sources"][1]["nameId"] = json!("DifferentId");
    assert_eq!(
        b,
        filtered(&q2),
        "labels and warnings must not depend on actual caller IDs"
    );
}

#[test]
fn single_digit_birth_day_and_vowel_values_do_not_bypass_filtering() {
    for birth in ["0001-01-01", "2000-01-01", "9999-12-31"] {
        let q = json!({"operation":"profile","name":{"given":"A"},"birthDate":birth,"referenceDate":"2026-01-01","years":1,
            "extensions":{"nameChallenges":{"nameOrder":"givenMiddleFamily"}}});
        let a = full(&q);
        let b = filtered(&q);
        assert!(b["data"]["birth"].get("birthDay").is_none());
        assert!(b["data"]["birth"]["attitude"].get("root").is_none());
        let c = &b["data"]["extended"]["nameChallenges"]["birthName"];
        assert!(c["soulUrge"].get("first").is_none());
        assert!(c["soulUrge"].get("last").is_none());
        assert_eq!(
            c["soulUrge"]["value"],
            a["data"]["extended"]["nameChallenges"]["birthName"]["soulUrge"]["value"]
        );
        assert_eq!(c["personality"]["status"], "unavailable");
        assert_eq!(c["personality"]["reason"], "noMatchingLetters");
        no_sensitive_fields(&b);
    }
}

#[test]
fn vietnamese_calendar_options_and_extensions_matrix_preserves_all_retained_numbers() {
    let mut checked = 0;
    for name in ["Đặng Thị Ý", "D\u{0301}ang", "A-Yay", "BCDF", "AEIOU", "A"] {
        for birth in [
            "0001-01-01",
            "1900-02-28",
            "2000-02-29",
            "1975-11-25",
            "9999-12-31",
        ] {
            for y in ["contextual", "vowel", "consonant"] {
                for keep in [false, true] {
                    for op in ["profile", "name", "birth", "cycles", "essence"] {
                        for ext in [
                            Value::Null,
                            json!({}),
                            json!({"reducedValues":false,"indicators":false}),
                        ] {
                            let q = json!({"operation":op,"name":{"given":name},"birthDate":birth,"referenceDate":"2026-11-13",
                                "startAge":120,"years":1,"options":{"keepMasterNumbers":keep,"yRule":y},"extensions":ext});
                            let a = full(&q);
                            let b = filtered(&q);
                            assert!(compare_numbers(&a["data"], &b["data"]) > 0);
                            no_sensitive_fields(&b);
                            assert_eq!(a["options"], b["options"]);
                            checked += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(checked, 2700);
}

#[test]
fn reference_date_traces_are_retained_and_explicitly_disclosed() {
    let q = json!({"operation":"cycles","birthDate":"1980-11-22","referenceDate":"2026-11-13"});
    let a = full(&q);
    let b = filtered(&q);
    compare_numbers(&a["data"], &b["data"]);
    let c = &b["data"]["cycles"];
    assert_eq!(c["universalYear"]["root"], 2026);
    assert_eq!(
        c["universalMonth"]["root"].as_u64().unwrap()
            - c["universalYear"]["value"].as_u64().unwrap(),
        11
    );
    assert_eq!(b["outputPolicy"]["referenceDateMayBeRecoverable"], true);
    let n = filtered(&json!({"operation":"name","name":{"given":"Ada"}}));
    assert_eq!(n["outputPolicy"]["referenceDateMayBeRecoverable"], false);
}

#[test]
fn error_validation_is_identical_and_does_not_echo_input_values() {
    for q in [
        json!({"operation":"name","name":{"given":"PRIVATE王"}}),
        json!({"operation":"names","names":[{"id":"PrivateIdentity","name":{"given":"A"}},{"id":"PrivateIdentity","name":{"given":"B"}}]}),
        json!({"operation":"birth","birthDate":"1900-02-29"}),
        json!({"operation":"essence","name":{"given":"A"},"birthDate":"2000-01-01","startAge":120,"years":2}),
    ] {
        let a: Value = serde_json::from_str(&calculate_json(&q.to_string())).unwrap();
        let b: Value = serde_json::from_str(&calculate_filtered_json(&q.to_string())).unwrap();
        assert_eq!(a, b);
        assert_eq!(b["ok"], false);
        assert!(!b.to_string().contains("PrivateIdentity"));
        assert!(!b.to_string().contains("PRIVATE王"));
    }
    for raw in [
        "",
        "null",
        "[]",
        "{}\0ignored",
        r#"{"operation":"birth","operation":"name"}"#,
    ] {
        assert_eq!(calculate_json(raw), calculate_filtered_json(raw));
    }
    for bytes in [vec![0xff], vec![b' '; 16385], b"{}\0ignored".to_vec()] {
        let v: Value = serde_json::from_str(&calculate_filtered_bytes(&bytes)).unwrap();
        assert_eq!(v["ok"], false);
    }
}

#[test]
fn parallel_full_and_filtered_calls_have_no_shared_labels_or_mutation() {
    let q = profile();
    let a = full(&q);
    let b = filtered(&q);
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let q = q.clone();
            let a = a.clone();
            let b = b.clone();
            std::thread::spawn(move || {
                for _ in 0..30 {
                    assert_eq!(full(&q), a);
                    assert_eq!(filtered(&q), b);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn maximum_names_groups_and_utf8_component_limits_are_preserved() {
    for given in ["A".repeat(1024), "Đ".repeat(512)] {
        let q = json!({"operation":"name","name":{"given":given}});
        let b = filtered(&q);
        compare_numbers(&full(&q)["data"], &b["data"]);
        no_sensitive_fields(&b);
    }
    let names: Vec<Value> = (0..32)
        .map(|i| json!({"id":format!("private{i}"),"name":{"given":"A".repeat(i+1)}}))
        .collect();
    let combinations:Vec<Value>=(0..32).map(|i|json!({"id":format!("secret{i}"),"sources":[{"nameId":"private0"},{"nameId":"private31"}]})).collect();
    let q = json!({"operation":"names","names":names,"combinations":combinations,"birthDate":"2000-01-01"});
    let a = full(&q);
    let b = filtered(&q);
    assert_eq!(b["data"]["names"]["names"][31]["id"], "n32");
    assert_eq!(b["data"]["names"]["combinations"][31]["id"], "c32");
    assert_eq!(
        b["data"]["names"]["combinations"][31]["sources"][1]["nameId"],
        "n32"
    );
    assert_eq!(compare_numbers(&a["data"], &b["data"]), 64 * 7);
    no_sensitive_fields(&b);
}
