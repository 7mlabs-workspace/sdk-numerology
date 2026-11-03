use numerology_core::{calculate_json, reduce};
use serde_json::{json, Value};
fn calc(request: Value) -> Value {
    serde_json::from_str(&calculate_json(&request.to_string())).unwrap()
}
fn profile() -> Value {
    json!({"operation":"profile","name":{"given":"Ada","family":"Lovelace"},"birthDate":"1815-12-10","referenceDate":"2030-04-05"})
}
#[test]
fn hand_worked_ada_profile() {
    let v = calc(profile());
    assert_eq!(v["ok"], true);
    let d = &v["result"]["data"];
    for (path, expected) in [
        ("/name/expression/root", 36),
        ("/name/expression/value", 9),
        ("/name/soulUrge/root", 19),
        ("/name/soulUrge/value", 1),
        ("/name/personality/root", 17),
        ("/name/personality/value", 8),
        ("/name/balance/value", 4),
        ("/birth/lifePath/value", 1),
        ("/cycles/personalYear/value", 9),
        ("/cycles/personalMonth/value", 4),
        ("/cycles/personalDay/value", 9),
        ("/cycles/universalYear/value", 5),
        ("/cycles/universalMonth/value", 9),
        ("/cycles/universalDay/value", 5),
        ("/derived/maturity/value", 1),
        ("/essence/years/0/essence/value", 4),
    ] {
        assert_eq!(d.pointer(path).unwrap(), expected, "{path}")
    }
    assert_eq!(
        d["cycles"]["pinnacles"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x["number"]["value"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        vec![4, 7, 11, 9]
    );
    assert_eq!(d["name"]["missingNumbers"], json!([2, 7, 8, 9]));
    assert_eq!(d["name"]["hiddenPassion"], json!([1, 3]));
    assert_eq!(d["name"]["soulUrge"]["karmicDebt"], 19);
}
#[test]
fn published_component_master_example() {
    let v = calc(json!({"operation":"birth","birthDate":"1975-11-25"}));
    let d = &v["result"]["data"]["birth"];
    assert_eq!(d["components"]["month"]["value"], 11);
    assert_eq!(d["components"]["year"]["value"], 22);
    assert_eq!(d["lifePath"]["root"], 40);
    assert_eq!(d["lifePath"]["value"], 4);
}
#[test]
fn third_pinnacle_uses_first_two_results() {
    let v =
        calc(json!({"operation":"cycles","birthDate":"2001-09-08","referenceDate":"2026-01-01"}));
    let p = &v["result"]["data"]["cycles"]["pinnacles"];
    assert_eq!(p[0]["number"]["value"], 8);
    assert_eq!(p[1]["number"]["value"], 11);
    assert_eq!(p[2]["number"]["root"], 19);
    assert_eq!(p[2]["number"]["karmicDebt"], 19);
    assert_eq!(p[3]["endAge"], Value::Null);
}
#[test]
fn regressions_prevent_original_hangs() {
    for name in ["12345", "---", "王明", ""] {
        let v = calc(
            json!({"operation":"profile","name":{"given":name},"birthDate":"2000-01-01","referenceDate":"2026-01-01"}),
        );
        assert_eq!(v["ok"], false, "{name}");
    }
}
#[test]
fn explicit_settings_change_results() {
    let v = calc(json!({"operation":"name","name":{"given":"Ly"},"options":{"yRule":"vowel"}}));
    let c = calc(json!({"operation":"name","name":{"given":"Ly"},"options":{"yRule":"consonant"}}));
    assert_eq!(v["result"]["data"]["name"]["soulUrge"]["root"], 7);
    assert_eq!(c["result"]["data"]["name"]["soulUrge"]["root"], 0);
    let v = calc(json!({"operation":"birth","birthDate":"1963-02-17"}));
    let c = calc(
        json!({"operation":"birth","birthDate":"1963-02-17","options":{"keepMasterNumbers":false}}),
    );
    assert_eq!(v["result"]["data"]["birth"]["lifePath"]["value"], 11);
    assert_eq!(c["result"]["data"]["birth"]["lifePath"]["value"], 2);
}
#[test]
fn normalization_and_no_mutation() {
    let mut a = profile();
    a["name"] = json!({"given":"Án","family":"Đặng"});
    let before = a.clone();
    let va = calc(a.clone());
    let mut b = a.clone();
    b["name"] = json!({"given":"An","family":"Dang"});
    assert_eq!(va, calc(b));
    assert_eq!(a, before);
    let again = calc(a);
    assert_eq!(again, va);
}
#[test]
fn reference_date_is_required_and_used() {
    let v = calc(json!({"operation":"cycles","birthDate":"2000-01-01"}));
    assert_eq!(v["error"]["code"], "MISSING_FIELD");
    let a = calc(profile());
    let mut b = profile();
    b["referenceDate"] = json!("2030-04-06");
    let b = calc(b);
    assert_ne!(
        a["result"]["data"]["cycles"]["personalDay"],
        b["result"]["data"]["cycles"]["personalDay"]
    );
}
#[test]
fn malformed_and_limits() {
    for raw in [
        "",
        "null",
        "[]",
        r#"{"operation":"birth","birthDate":"2000-01-01","options":{"keepMasterNumbers":true,"keepMasterNumbers":false}}"#,
        r#"{"operation":"name","name":{"given":"Ada","extra":1}}"#,
    ] {
        let v: Value = serde_json::from_str(&calculate_json(raw)).unwrap();
        assert_eq!(v["ok"], false)
    }
    let v: Value = serde_json::from_str(&calculate_json(&" ".repeat(16385))).unwrap();
    assert_eq!(v["error"]["code"], "REQUEST_TOO_LARGE");
    let mut q = profile();
    q["years"] = json!(121);
    assert_eq!(calc(q)["error"]["code"], "INVALID_RANGE");
}
#[test]
fn all_supported_calendar_boundaries() {
    for year in [
        1, 4, 100, 400, 1600, 1700, 1800, 1900, 2000, 2024, 2100, 2400, 9999,
    ] {
        for month in 1..=12 {
            let max = match month {
                2 => {
                    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                        29
                    } else {
                        28
                    }
                }
                4 | 6 | 9 | 11 => 30,
                _ => 31,
            };
            for day in [1, max] {
                let birth = format!("{year:04}-{month:02}-{day:02}");
                let v = calc(json!({"operation":"birth","birthDate":birth}));
                assert_eq!(v["ok"], true, "{birth}")
            }
            let v = calc(
                json!({"operation":"birth","birthDate":format!("{year:04}-{month:02}-{:02}",max+1)}),
            );
            assert_eq!(v["ok"], false);
        }
    }
}
#[test]
fn reduction_properties_exhaustive() {
    for n in 0..100000 {
        for keep in [false, true] {
            let r = reduce(n, keep);
            assert!(r.value < 10 || keep && [11, 22, 33].contains(&r.value));
            assert_eq!(r.root, n);
            assert_eq!(*r.steps.last().unwrap(), r.value);
            if n > 0 {
                assert_eq!(
                    if n % 9 == 0 { 9 } else { n % 9 },
                    if r.value % 9 == 0 { 9 } else { r.value % 9 }
                );
            }
        }
    }
}
#[test]
fn concurrency_is_deterministic() {
    let expected = calc(profile());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let expected = expected.clone();
            std::thread::spawn(move || {
                for _ in 0..100 {
                    assert_eq!(calc(profile()), expected)
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
#[test]
fn modular_operations_match_profile() {
    let all = calc(profile());
    for (op, section) in [
        ("name", "name"),
        ("birth", "birth"),
        ("cycles", "cycles"),
        ("essence", "essence"),
    ] {
        let mut q = profile();
        q["operation"] = json!(op);
        assert_eq!(
            calc(q)["result"]["data"][section],
            all["result"]["data"][section]
        );
    }
}
