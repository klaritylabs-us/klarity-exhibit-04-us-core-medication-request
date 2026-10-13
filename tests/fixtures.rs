use std::fs;
use std::path::PathBuf;

use us_core_medication_request::validate_medication_request_json;

fn fixture(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push(name);
    fs::read_to_string(path).expect("fixture")
}

#[test]
fn valid_order() {
    let report = validate_medication_request_json(&fixture("valid-order.json")).unwrap();
    assert!(report.is_valid(), "{:?}", report.issues);
}

#[test]
fn ndc_only_fails() {
    let report = validate_medication_request_json(&fixture("invalid-ndc-only.json")).unwrap();
    assert!(!report.is_valid());
}

#[test]
fn missing_intent_fails() {
    let report = validate_medication_request_json(&fixture("invalid-missing-intent.json")).unwrap();
    assert!(!report.is_valid());
}

#[test]
fn missing_rxnorm_fails() {
    let report = validate_medication_request_json(&fixture("invalid-missing-rxnorm.json")).unwrap();
    assert!(!report.is_valid());
}
