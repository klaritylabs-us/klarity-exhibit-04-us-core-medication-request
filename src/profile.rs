//! US Core MedicationRequest STU 6.1.

pub const US_CORE_MEDICATION_REQUEST_PROFILE: &str =
    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-medicationrequest";

pub const RXNORM_SYSTEM: &str = "http://www.nlm.nih.gov/research/umls/rxnorm";

/// NDC often appears in pharmacy feeds — not a substitute for RxNorm on US Core.
pub const NDC_SYSTEM: &str = "http://hl7.org/fhir/sid/ndc";
