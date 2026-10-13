//! Validate a US Core MedicationRequest JSON file.

use std::env;
use std::fs;
use std::process::ExitCode;

use us_core_medication_request::validate_medication_request_json;

fn main() -> ExitCode {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: validate-medication-request <path-to-medicationrequest.json>");
            return ExitCode::from(2);
        }
    };

    let json = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return ExitCode::from(2);
        }
    };

    let report = match validate_medication_request_json(&json) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(2);
        }
    };

    let pretty = serde_json::to_string_pretty(&report.issues).unwrap_or_else(|_| "[]".into());
    println!("{}", pretty);

    if report.is_valid() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
