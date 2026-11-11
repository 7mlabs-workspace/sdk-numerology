use numerology_core::{
    calculate_json, calculate_names, calculate_with_name_set, Extensions, Name, NameInput, NameSet,
    NamesRequest, Options, Request,
};
use serde_json::{json, Value};

fn calc(q: Value) -> Value {
    serde_json::from_str(&calculate_json(&q.to_string())).unwrap()
}
fn input() -> Value {
    json!({"operation":"names","birthDate":"2006-02-08","names":[
        {"id":"legal","name":{"given":"HENIT","middle":"RUH UTAN","family":"IUB"}},
        {"id":"nick","name":{"given":"MAP"}}
    ],"combinations":[{"id":"given-nick","sources":[{"nameId":"legal","part":"given"},{"nameId":"nick"}]}]})
}
fn profile() -> Value {
    json!({"operation":"profile","birthDate":"2006-02-08","referenceDate":"2026-11-11","years":1,
      "name":{"given":"HENIT","middle":"RUH UTAN","family":"IUB"},
      "extensions":{"currentName":{"given":"A"}}})
}

#[test]
fn names_reuse_single_name_core_and_preserve_input_order() {
    let q = input();
    let v = calc(q.clone());
    assert_eq!(v["ok"], true);
    assert_eq!(v["result"]["schemaVersion"], "1.1");
    assert_eq!(
        v["result"]["namesRulesVersion"],
        "pythagorean-7mlabs-names-1.0"
    );
    let rows = v["result"]["data"]["names"]["names"].as_array().unwrap();
    assert_eq!(
        rows.iter()
            .map(|r| r["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["legal", "nick"]
    );
    for (i, r) in rows.iter().enumerate() {
        let single = calc(
            json!({"operation":"name","name":q["names"][i]["name"],"extensions":{"indicators":false}}),
        );
        assert_eq!(r["numbers"], single["result"]["data"]["name"]);
        assert_eq!(r["source"], "input");
    }
}

#[test]
fn thoughts_are_scoped_to_each_given_name_or_the_selected_group_text() {
    let v = calc(input());
    let set = &v["result"]["data"]["names"];
    assert_eq!(set["names"][0]["rationalThought"]["number"]["value"], 1);
    assert_eq!(set["names"][1]["rationalThought"]["number"]["value"], 2);
    let group = &set["combinations"][0];
    assert_eq!(group["method"], "concatenateSelectedParts");
    assert_eq!(group["reduction"], "wholeNameSum");
    assert_eq!(
        group["sources"][0],
        json!({"nameId":"legal","part":"given","text":"HENIT"})
    );
    assert_eq!(
        group["sources"][1],
        json!({"nameId":"nick","part":"full","text":"MAP"})
    );
    assert_eq!(group["numbers"]["expression"]["root"], 41);
    assert_eq!(group["numbers"]["soulUrge"]["value"], 6);
    assert_eq!(group["numbers"]["personality"]["value"], 8);
    assert_eq!(group["rationalThought"]["nameSource"], "selectedText");
    assert_eq!(group["rationalThought"]["number"]["root"], 49);
    assert_eq!(group["rationalThought"]["number"]["value"], 4);
}

#[test]
fn no_birth_date_means_no_thought_and_no_groups_are_generated_implicitly() {
    let mut q = input();
    q.as_object_mut().unwrap().remove("birthDate");
    q.as_object_mut().unwrap().remove("combinations");
    let v = calc(q);
    assert_eq!(v["ok"], true);
    let set = &v["result"]["data"]["names"];
    assert_eq!(set["names"].as_array().unwrap().len(), 2);
    assert!(set["names"][0].get("rationalThought").is_none());
    assert_eq!(set["combinations"], json!([]));
}

#[test]
fn group_reduces_the_unreduced_sum_instead_of_summing_single_results() {
    let mut q = json!({"operation":"names","names":[{"id":"a","name":{"given":"AIIII"}},{"id":"b","name":{"given":"A"}}],
        "combinations":[{"id":"combined","sources":[{"nameId":"a"},{"nameId":"b"}]}]});
    let v = calc(q.clone());
    let set = &v["result"]["data"]["names"];
    assert_eq!(set["names"][0]["numbers"]["expression"]["value"], 1);
    assert_eq!(set["names"][1]["numbers"]["expression"]["value"], 1);
    assert_eq!(set["combinations"][0]["numbers"]["expression"]["root"], 38);
    assert_eq!(set["combinations"][0]["numbers"]["expression"]["value"], 11);
    assert_eq!(
        set["combinations"][0]["numbers"]["expression"]["reducedValue"],
        2
    );
    q["options"] = json!({"keepMasterNumbers":false});
    assert_eq!(
        calc(q)["result"]["data"]["names"]["combinations"][0]["numbers"]["expression"]["steps"],
        json!([38, 11, 2])
    );
}

#[test]
fn normalized_duplicate_names_are_kept_individually_but_not_double_counted_in_a_group() {
    let mut q = json!({"operation":"names","names":[{"id":"a","name":{"given":"Án"}},{"id":"b","name":{"given":"An"}}]});
    let v = calc(q.clone());
    let rows = &v["result"]["data"]["names"]["names"];
    assert_eq!(rows[0]["numbers"], rows[1]["numbers"]);
    q["combinations"] = json!([{"id":"both","sources":[{"nameId":"a"},{"nameId":"b"}]}]);
    assert_eq!(calc(q)["error"]["code"], "DUPLICATE_NAME_SOURCE");
}

#[test]
fn groups_reject_overlapping_parts_but_allow_disjoint_parts_in_caller_order() {
    let mut q = input();
    q["combinations"][0]["sources"] = json!([{"nameId":"legal"},{"nameId":"legal","part":"given"}]);
    assert_eq!(calc(q.clone())["error"]["code"], "DUPLICATE_NAME_SOURCE");
    let duplicate = q["names"][0]["name"].clone();
    q["names"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"duplicate","name":duplicate}));
    q["combinations"][0]["sources"] =
        json!([{"nameId":"legal"},{"nameId":"duplicate","part":"given"}]);
    assert_eq!(calc(q.clone())["error"]["code"], "DUPLICATE_NAME_SOURCE");
    q["combinations"][0]["sources"] =
        json!([{"nameId":"legal","part":"family"},{"nameId":"legal","part":"given"}]);
    let v = calc(q);
    assert_eq!(
        v["result"]["data"]["names"]["combinations"][0]["numbers"]["normalizedName"]["given"],
        "IUB HENIT"
    );
}

#[test]
fn all_ids_are_unique_and_groups_cannot_reference_other_groups() {
    let mut q = input();
    q["names"][1]["id"] = json!("legal");
    assert_eq!(calc(q)["error"]["code"], "DUPLICATE_ID");
    let mut q = input();
    q["combinations"][0]["id"] = json!("legal");
    assert_eq!(calc(q)["error"]["code"], "DUPLICATE_ID");
    let mut q = input();
    let duplicate = q["combinations"][0].clone();
    q["combinations"].as_array_mut().unwrap().push(duplicate);
    assert_eq!(calc(q)["error"]["code"], "DUPLICATE_ID");
    let mut q = input();
    q["combinations"][0]["sources"][1]["nameId"] = json!("given-nick");
    let v = calc(q);
    assert_eq!(v["error"]["code"], "UNKNOWN_NAME_ID");
    assert_eq!(v["error"]["field"], "combinations[0].sources[1].nameId");
}

#[test]
fn empty_parts_invalid_ids_and_unsupported_name_text_report_precise_fields() {
    let mut q = input();
    q["combinations"][0]["sources"][1]["part"] = json!("family");
    assert_eq!(calc(q)["error"]["code"], "EMPTY_NAME_PART");
    for id in ["", "-start", "two names", "字", &"x".repeat(65)] {
        let mut q = input();
        q["names"][0]["id"] = json!(id);
        assert_eq!(calc(q)["error"]["field"], "names[0].id");
    }
    for name in ["123", "王明", "---", ""] {
        let mut q = input();
        q["names"][1]["name"]["given"] = json!(name);
        assert_eq!(calc(q)["error"]["field"], "names[1].name.given");
    }
}

#[test]
fn collection_counts_and_combined_text_are_bounded() {
    let all_names: Vec<Value> = (0..32)
        .map(|i| json!({"id":format!("n{i}"),"name":{"given":format!("A{}","B".repeat(i))}}))
        .collect();
    let all_sources: Vec<Value> = (0..32).map(|i| json!({"nameId":format!("n{i}")})).collect();
    let all = calc(
        json!({"operation":"names","names":all_names,"combinations":[{"id":"all","sources":all_sources}]}),
    );
    assert_eq!(all["ok"], true);
    assert_eq!(
        all["result"]["data"]["names"]["combinations"][0]["sources"]
            .as_array()
            .unwrap()
            .len(),
        32
    );
    assert_eq!(
        all["result"]["data"]["names"]["combinations"][0]["numbers"]["expression"]["root"],
        1024
    );
    let mut q = input();
    q["names"] = json!([]);
    assert_eq!(calc(q)["error"]["code"], "INVALID_RANGE");
    let names: Vec<Value> = (0..32)
        .map(|i| json!({"id":format!("n{i}"),"name":{"given":"Ada"}}))
        .collect();
    let mut q = json!({"operation":"names","names":names});
    assert_eq!(calc(q.clone())["ok"], true);
    q["names"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"n32","name":{"given":"Ada"}}));
    assert_eq!(calc(q)["error"]["code"], "INVALID_RANGE");
    let mut q = input();
    q["combinations"] = Value::Array(
        (0..33)
            .map(|i| json!({"id":format!("g{i}"),"sources":[]}))
            .collect(),
    );
    assert_eq!(calc(q)["error"]["field"], "combinations");
    for count in [0, 1, 33] {
        let mut q = input();
        q["combinations"][0]["sources"] =
            Value::Array((0..count).map(|_| json!({"nameId":"nick"})).collect());
        assert_eq!(calc(q)["error"]["field"], "combinations[0].sources");
    }
    let mut q = json!({"operation":"names","names":[{"id":"a","name":{"given":"A".repeat(1024),"middle":"B".repeat(1024),"family":"C".repeat(1024)}},{"id":"b","name":{"given":"D".repeat(1021)}}],"combinations":[{"id":"combined","sources":[{"nameId":"a"},{"nameId":"b"}]}]});
    assert_eq!(calc(q.clone())["ok"], true);
    q["names"][1]["name"]["given"] = json!("D".repeat(1022));
    assert_eq!(calc(q)["error"]["code"], "INPUT_TOO_LONG");
}

#[test]
fn strict_wire_validation_is_preserved_for_new_fields() {
    for raw in [
        r#"{"operation":"names","names":[],"names":[]}"#,
        r#"{"operation":"names","names":null}"#,
        r#"{"operation":"names","names":[{"id":"a","name":{"given":"Ada"},"weight":2}]}"#,
        r#"{"operation":"names","names":[{"id":"a","name":{"given":"Ada"}}],"combinations":null}"#,
        r#"{"operation":"names","names":[{"id":"a","name":{"given":"Ada"}}],"combinations":[{"id":"g","sources":[{"nameId":"a","part":"nickname"}]}]}"#,
    ] {
        let v: Value = serde_json::from_str(&calculate_json(raw)).unwrap();
        assert_eq!(v["error"]["code"], "INVALID_REQUEST", "{raw}");
    }
    let mut q = input();
    q["system"] = json!("chaldean");
    assert_eq!(calc(q)["error"]["code"], "UNSUPPORTED_SYSTEM");
    let mut q = input();
    q["birthDate"] = json!("1900-02-29");
    assert_eq!(calc(q)["error"]["code"], "INVALID_DATE");
    let mut q = input();
    q["name"] = json!({"given":"Ignored"});
    assert_eq!(calc(q)["error"]["code"], "INVALID_REQUEST");
    let mut q = input();
    q["operation"] = json!("name");
    q["name"] = json!({"given":"Ada"});
    assert_eq!(calc(q)["error"]["code"], "INVALID_REQUEST");
}

#[test]
fn profile_name_set_is_opt_in_and_keeps_all_preexisting_values_and_current_name() {
    let mut q = profile();
    let old = calc(q.clone());
    q["extensions"]["nameSet"] = json!({"names":[{"id":"nick","name":{"given":"MAP"}}],"combinations":[{"id":"given-nick","sources":[{"nameId":"birth","part":"given"},{"nameId":"nick"}]}]});
    let mut v = calc(q.clone());
    assert_eq!(v["ok"], true);
    let set = &v["result"]["data"]["extended"]["nameSet"];
    assert_eq!(set["names"][0]["id"], "birth");
    assert_eq!(set["names"][0]["source"], "profile.birthName");
    assert_eq!(set["names"][0]["numbers"], old["result"]["data"]["name"]);
    assert_eq!(
        set["combinations"][0]["rationalThought"]["number"]["value"],
        4
    );
    v["result"]["data"]["extended"]
        .as_object_mut()
        .unwrap()
        .remove("nameSet");
    v["result"]["extensions"]
        .as_object_mut()
        .unwrap()
        .remove("namesRulesVersion");
    assert_eq!(v, old);
    q["extensions"]["reducedValues"] = json!(false);
    let v = calc(q);
    assert!(
        v["result"]["data"]["extended"]["nameSet"]["names"][0]["numbers"]["soulUrge"]
            .get("reducedValue")
            .is_none()
    );
}

#[test]
fn profile_reserves_birth_and_rejects_ignored_name_sets() {
    let mut q = profile();
    q["extensions"]["nameSet"] = json!({});
    assert_eq!(
        calc(q.clone())["result"]["data"]["extended"]["nameSet"]["names"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    q["extensions"]["nameSet"] = json!({"names":[{"id":"birth","name":{"given":"Other"}}]});
    assert_eq!(calc(q)["error"]["code"], "RESERVED_NAME_ID");
    let mut q = profile();
    q["extensions"]["nameSet"] = json!({});
    q["extensions"]["indicators"] = json!(false);
    assert_eq!(calc(q)["error"]["code"], "INVALID_EXTENSION");
    let mut q = profile();
    q["extensions"]["nameSet"] = json!({});
    q["operation"] = json!("name");
    assert_eq!(calc(q)["error"]["field"], "extensions.nameSet");
    let mut q = profile();
    q["extensions"]["nameSet"] = json!({"names":(0..31).map(|i|json!({"id":format!("n{i}"),"name":{"given":"Ada"}})).collect::<Vec<_>>()});
    assert_eq!(
        calc(q.clone())["result"]["data"]["extended"]["nameSet"]["names"]
            .as_array()
            .unwrap()
            .len(),
        32
    );
    q["extensions"]["nameSet"]["names"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"n31","name":{"given":"Ada"}}));
    assert_eq!(calc(q)["error"]["code"], "INVALID_RANGE");
}

#[test]
fn group_word_boundaries_preserve_y_policy_and_request_is_not_mutated() {
    let mut q = json!({"operation":"names","names":[{"id":"a","name":{"given":"Ly"}},{"id":"b","name":{"given":"Yan"}}],"combinations":[{"id":"together","sources":[{"nameId":"a"},{"nameId":"b"}]}]});
    let before = q.clone();
    let v = calc(q.clone());
    assert_eq!(q, before);
    let expected = calc(
        json!({"operation":"name","name":{"given":"LY YAN"},"extensions":{"indicators":false}}),
    );
    assert_eq!(
        v["result"]["data"]["names"]["combinations"][0]["numbers"],
        expected["result"]["data"]["name"]
    );
    assert_eq!(v["result"]["warnings"].as_array().unwrap().len(), 2);
    q["options"] = json!({"yRule":"consonant"});
    assert_eq!(calc(q)["result"]["warnings"], json!([]));
}

#[test]
fn public_rust_apis_keep_previous_struct_literals_and_offer_names_separately() {
    let request = Request {
        operation: "profile".into(),
        system: "pythagorean".into(),
        name: Some(Name {
            given: "Ada".into(),
            middle: String::new(),
            family: String::new(),
        }),
        birth_date: Some("2000-02-29".into()),
        reference_date: Some("2026-11-11".into()),
        options: Options::default(),
        start_age: 0,
        years: 1,
    };
    let extensions = Extensions {
        reduced_values: true,
        indicators: true,
        current_name: None,
    };
    let set = NameSet {
        names: vec![NameInput {
            id: "nick".into(),
            name: Name {
                given: "Map".into(),
                ..Name::default()
            },
        }],
        combinations: vec![],
    };
    assert_eq!(
        calculate_with_name_set(&request, &extensions, &set).unwrap()["data"]["extended"]
            ["nameSet"]["names"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    let names = NamesRequest {
        names: set.names,
        combinations: vec![],
        birth_date: None,
        system: "pythagorean".into(),
        options: Options::default(),
    };
    let before = serde_json::to_value(&names).unwrap();
    assert_eq!(calculate_names(&names).unwrap()["operation"], "names");
    assert_eq!(serde_json::to_value(&names).unwrap(), before);
}

#[test]
fn names_and_combinations_are_deterministic_under_concurrency() {
    let expected = calc(input());
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let expected = expected.clone();
            std::thread::spawn(move || {
                for _ in 0..40 {
                    assert_eq!(calc(input()), expected);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
