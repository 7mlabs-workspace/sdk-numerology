use numerology_core::{
    calculate_json, calculate_name_challenges, Name, NameChallengeOptions, NameOrder, Options,
};
use serde_json::{json, Value};

fn calc(q: Value) -> Value {
    serde_json::from_str(&calculate_json(&q.to_string())).unwrap()
}
fn request(given: &str) -> Value {
    json!({"operation":"name","name":{"given":given},"extensions":{"nameChallenges":{"nameOrder":"givenMiddleFamily"}}})
}
fn result(v: &Value) -> &Value {
    &v["result"]["data"]["extended"]["nameChallenges"]["birthName"]
}

#[test]
fn independently_published_worked_examples() {
    // Numerologist.com, Change Your Life manual, p29: David Edward Hanson, A(1) / O(6).
    let v = calc(request("David Edward Hanson"));
    let s = &result(&v)["soulUrge"];
    assert_eq!(s["value"], 5);
    assert_eq!(s["first"], json!({"letter":"A","value":1}));
    assert_eq!(s["last"], json!({"letter":"O","value":6}));
    // Roy Kirkland's published interview: Dave Jones D(4) / S(1) -> 3.
    let v = calc(request("Dave Jones"));
    assert_eq!(result(&v)["personality"]["difference"], 3);
    assert_eq!(result(&v)["personality"]["value"], 3);
    let v = calc(request("Steve Jones"));
    assert_eq!(result(&v)["personality"]["difference"], 0);
    assert_eq!(result(&v)["personality"]["value"], 9);
}

#[test]
fn order_is_explicit_and_changes_only_the_new_namespace() {
    let mut q = request("EON");
    q["name"] = json!({"given":"EON","middle":"RAI","family":"BU"});
    let a = calc(q.clone());
    q["extensions"]["nameChallenges"]["nameOrder"] = json!("familyMiddleGiven");
    let b = calc(q.clone());
    assert_eq!(result(&a)["orderedText"], "EON RAI BU");
    assert_eq!(result(&b)["orderedText"], "BU RAI EON");
    assert_eq!(result(&a)["soulUrge"]["value"], 2); // E5 to U3
    assert_eq!(result(&b)["soulUrge"]["value"], 3); // U3 to O6
    assert_eq!(result(&a)["personality"]["value"], 3); // N5 to B2
    assert_eq!(result(&b)["personality"]["value"], 3);
    assert_eq!(a["result"]["data"]["name"], b["result"]["data"]["name"]);
    q["extensions"] = json!({});
    let old = calc(q);
    let mut stripped = b;
    stripped["result"]["data"]["extended"]
        .as_object_mut()
        .unwrap()
        .remove("nameChallenges");
    stripped["result"]["extensions"]
        .as_object_mut()
        .unwrap()
        .remove("nameChallengesRulesVersion");
    assert_eq!(stripped, old);
}

#[test]
fn missing_letters_are_unavailable_but_a_single_letter_is_a_valid_pair() {
    for (text, absent, present, value) in [
        ("BCDF", "soulUrge", "personality", 4),
        ("AEIOU", "personality", "soulUrge", 2),
        ("A", "personality", "soulUrge", 0),
        ("B", "soulUrge", "personality", 9),
    ] {
        let v = calc(request(text));
        let r = result(&v);
        assert_eq!(r[absent]["status"], "unavailable");
        assert_eq!(r[absent]["value"], Value::Null);
        assert_eq!(r[absent]["difference"], Value::Null);
        assert_eq!(r[present]["status"], "available");
        assert_eq!(r[present]["value"], value);
    }
    let v = calc(request("ANNA"));
    assert_eq!(result(&v)["soulUrge"]["value"], 0);
}

#[test]
fn y_normalization_and_master_switch_are_explicit() {
    for rule in ["vowel", "consonant", "contextual"] {
        let mut q = request("Ý-BY");
        q["options"] = json!({"yRule":rule});
        let a = calc(q.clone());
        q["options"]["keepMasterNumbers"] = json!(false);
        let b = calc(q);
        assert_eq!(result(&a), result(&b));
        assert_eq!(result(&a)["orderedText"], "Y BY");
        // Vowel Y: Y7/Y7=0; consonant Y: vowels absent; contextual both vowel.
        if rule == "consonant" {
            assert_eq!(result(&a)["soulUrge"]["status"], "unavailable");
            assert_eq!(result(&a)["personality"]["value"], 9);
        } else {
            assert_eq!(result(&a)["soulUrge"]["value"], 0);
            assert_eq!(result(&a)["personality"]["value"], 9);
        }
        let mut q = request("YAEI");
        q["options"] = json!({"yRule":rule});
        let v = calc(q);
        assert_eq!(
            result(&v)["soulUrge"]["value"],
            if rule == "vowel" { 2 } else { 8 }
        );
        assert_eq!(
            result(&v)["personality"]["status"],
            if rule == "vowel" {
                "unavailable"
            } else {
                "available"
            }
        );
    }
}

#[test]
fn current_name_and_multiple_name_set_remain_separate() {
    let mut q = request("David Edward Hanson");
    q["operation"] = json!("profile");
    q["birthDate"] = json!("2000-02-29");
    q["referenceDate"] = json!("2026-11-11");
    q["years"] = json!(1);
    q["extensions"]["currentName"] = json!({"given":"Steve Jones"});
    q["extensions"]["nameSet"] = json!({"names":[{"id":"alias","name":{"given":"MAP"}}]});
    let mut a = calc(q.clone());
    assert_eq!(a["ok"], true);
    assert_eq!(
        a["result"]["data"]["extended"]["nameChallenges"]["currentName"]["personality"]["value"],
        9
    );
    assert_eq!(result(&a)["soulUrge"]["value"], 5);
    a["result"]["data"]["extended"]
        .as_object_mut()
        .unwrap()
        .remove("nameChallenges");
    a["result"]["extensions"]
        .as_object_mut()
        .unwrap()
        .remove("nameChallengesRulesVersion");
    q["extensions"]
        .as_object_mut()
        .unwrap()
        .remove("nameChallenges");
    assert_eq!(a, calc(q));
}

#[test]
fn invalid_settings_and_unsupported_operations_are_rejected() {
    for setting in [
        json!({}),
        json!({"nameOrder":"guess"}),
        json!({"nameOrder":true}),
        json!({"nameOrder":"givenMiddleFamily","typo":true}),
    ] {
        let mut q = request("A");
        q["extensions"]["nameChallenges"] = setting;
        assert_eq!(calc(q)["error"]["code"], "INVALID_REQUEST");
    }
    for op in ["birth", "cycles", "essence"] {
        let mut q = request("A");
        q["operation"] = json!(op);
        q["birthDate"] = json!("2000-01-01");
        q["referenceDate"] = json!("2026-01-01");
        assert_eq!(calc(q)["error"]["field"], "extensions.nameChallenges");
    }
    let mut q = request("A");
    q["extensions"]["indicators"] = json!(false);
    assert_eq!(calc(q)["error"]["code"], "INVALID_EXTENSION");
    let mut q = request("A");
    q["extensions"]["nameChallenges"] = Value::Null;
    let a = calc(q.clone());
    q["extensions"] = json!({});
    assert_eq!(a, calc(q));
    let raw = r#"{"operation":"name","name":{"given":"A"},"extensions":{"nameChallenges":{"nameOrder":"givenMiddleFamily","nameOrder":"familyMiddleGiven"}}}"#;
    assert_eq!(
        serde_json::from_str::<Value>(&calculate_json(raw)).unwrap()["error"]["code"],
        "INVALID_REQUEST"
    );
}

#[test]
fn public_rust_helper_and_all_endpoint_values() {
    // Exhaust all 26² letter pairs using an independent fixed Pythagorean table.
    let table = [
        1u32, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8,
    ];
    for first in b'A'..=b'Z' {
        for last in b'A'..=b'Z' {
            let is_vowel = |c: u8| b"AEIOU".contains(&c);
            if is_vowel(first) != is_vowel(last) {
                continue;
            }
            let n = Name {
                given: format!("{} {}", first as char, last as char),
                ..Name::default()
            };
            let options = Options {
                y_rule: numerology_core::YRule::Consonant,
                ..Options::default()
            };
            let actual = calculate_name_challenges(
                &n,
                &options,
                &NameChallengeOptions {
                    name_order: NameOrder::GivenMiddleFamily,
                },
            )
            .unwrap();
            let d = table[(first - b'A') as usize].abs_diff(table[(last - b'A') as usize]);
            let key = if is_vowel(first) {
                "soulUrge"
            } else {
                "personality"
            };
            assert_eq!(actual[key]["difference"], d);
            assert_eq!(
                actual[key]["value"],
                if key == "personality" && d == 0 { 9 } else { d }
            );
        }
    }
}
