//! US Core MedicationRequest — RxNorm, intent, and status checks.

use serde_json::Value;

use crate::outcome::{ValidationIssue, ValidationReport};
use crate::profile::{NDC_SYSTEM, RXNORM_SYSTEM, US_CORE_MEDICATION_REQUEST_PROFILE};

pub fn validate_medication_request_json(json: &str) -> Result<ValidationReport, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| format!("JSON parse: {e}"))?;
    Ok(validate_medication_request_value(&value))
}

pub fn validate_medication_request_value(mr: &Value) -> ValidationReport {
    let mut report = ValidationReport::ok();

    if mr.get("resourceType").and_then(Value::as_str) != Some("MedicationRequest") {
        report.push(ValidationIssue::error(
            "MedicationRequest.resourceType",
            "resourceType must be MedicationRequest",
            "medication-request-resource-type",
        ));
        return report;
    }

    check_meta_profile(mr, &mut report);
    check_status(mr, &mut report);
    check_intent(mr, &mut report);
    check_medication_rxnorm(mr, &mut report);
    check_subject(mr, &mut report);
    check_status_intent_combo(mr, &mut report);

    report
}

fn check_meta_profile(mr: &Value, report: &mut ValidationReport) {
    let has = mr
        .pointer("/meta/profile")
        .and_then(Value::as_array)
        .is_some_and(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .any(|p| p == US_CORE_MEDICATION_REQUEST_PROFILE)
        });

    if !has {
        report.push(ValidationIssue::error(
            "MedicationRequest.meta.profile",
            format!("meta.profile should include {}", US_CORE_MEDICATION_REQUEST_PROFILE),
            "us-core-meta-profile",
        ));
    }
}

fn check_status(mr: &Value, report: &mut ValidationReport) {
    let status = mr.get("status").and_then(Value::as_str);
    if status.is_none() {
        report.push(ValidationIssue::error(
            "MedicationRequest.status",
            "status is required",
            "status-required",
        ));
        return;
    }

    const ALLOWED: &[&str] = &[
        "active",
        "on-hold",
        "cancelled",
        "completed",
        "entered-in-error",
        "stopped",
        "draft",
        "unknown",
    ];
    if !ALLOWED.contains(&status.unwrap()) {
        report.push(ValidationIssue::warning(
            "MedicationRequest.status",
            format!("unexpected status value {:?}", status),
            "status-value",
        ));
    }
}

fn check_intent(mr: &Value, report: &mut ValidationReport) {
    let intent = mr.get("intent").and_then(Value::as_str);
    if intent.is_none() {
        report.push(ValidationIssue::error(
            "MedicationRequest.intent",
            "intent is required (order, plan, proposal, etc.)",
            "intent-required",
        ));
        return;
    }

    if intent == Some("plan") {
        report.push(ValidationIssue::warning(
            "MedicationRequest.intent",
            "intent plan is valid FHIR but many US exchanges expect order for actionable prescriptions",
            "intent-plan-vs-order",
        ));
    }
}

fn medication_codings(mr: &Value) -> Option<&Vec<Value>> {
    mr.pointer("/medicationCodeableConcept/coding")
        .and_then(Value::as_array)
}

fn check_medication_rxnorm(mr: &Value, report: &mut ValidationReport) {
    // US Core also allows medicationReference; this validator focuses on CodeableConcept paths.
    if mr.get("medicationReference").is_some() {
        return;
    }

    let codings = medication_codings(mr);
    if codings.is_none() {
        report.push(ValidationIssue::error(
            "MedicationRequest.medicationCodeableConcept",
            "medicationCodeableConcept or medicationReference is required",
            "medication-required",
        ));
        return;
    }

    let arr = codings.unwrap();
    let has_rxnorm = arr.iter().any(|c| {
        c.get("system").and_then(Value::as_str) == Some(RXNORM_SYSTEM)
            && c.get("code")
                .and_then(Value::as_str)
                .is_some_and(|s| !s.is_empty())
    });

    let ndc_only = arr.iter().any(|c| c.get("system").and_then(Value::as_str) == Some(NDC_SYSTEM))
        && !has_rxnorm;

    if ndc_only {
        report.push(ValidationIssue::error(
            "MedicationRequest.medicationCodeableConcept",
            "NDC-only medication coding is a common pharmacy mapper output; US Core expects RxNorm",
            "rxnorm-required",
        ));
        return;
    }

    if !has_rxnorm {
        report.push(ValidationIssue::error(
            "MedicationRequest.medicationCodeableConcept",
            format!("include RxNorm coding ({})", RXNORM_SYSTEM),
            "rxnorm-required",
        ));
    }
}

fn check_subject(mr: &Value, report: &mut ValidationReport) {
    let has_ref = mr
        .pointer("/subject/reference")
        .and_then(Value::as_str)
        .is_some_and(|r| !r.is_empty());

    if !has_ref {
        report.push(ValidationIssue::error(
            "MedicationRequest.subject",
            "subject.reference is required",
            "subject-required",
        ));
    }
}

fn check_status_intent_combo(mr: &Value, report: &mut ValidationReport) {
    let status = mr.get("status").and_then(Value::as_str);
    let intent = mr.get("intent").and_then(Value::as_str);

    if status == Some("completed") && intent == Some("proposal") {
        report.push(ValidationIssue::warning(
            "MedicationRequest",
            "completed status with proposal intent is unusual — check ambulatory export mapping",
            "status-intent-mismatch",
        ));
    }

    if status == Some("active") && intent == Some("proposal") {
        report.push(ValidationIssue::warning(
            "MedicationRequest",
            "active + proposal may be intentional CDS; confirm receiver expects actionable meds",
            "active-proposal",
        ));
    }
}
